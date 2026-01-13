package com.nearclip.service

import android.annotation.SuppressLint
import android.bluetooth.*
import android.bluetooth.le.*
import android.content.Context
import android.os.ParcelUuid
import android.util.Log
import kotlinx.coroutines.*
import java.util.*
import java.util.concurrent.ConcurrentHashMap

/**
 * Simplified BLE Manager for NearClip Android.
 * Hardware abstraction layer only - business logic is handled by Rust BleController.
 * Supports both Central (scanner) and Peripheral (advertiser) modes.
 */
@SuppressLint("MissingPermission")
class BleManager(private val context: Context) {

    companion object {
        private const val TAG = "BleManager"

        // NearClip BLE Service and Characteristic UUIDs
        // Must match the UUIDs defined in nearclip-ble/src/gatt.rs
        val SERVICE_UUID: UUID = UUID.fromString("4E454152-434C-4950-0000-000000000001")
        val DEVICE_ID_UUID: UUID = UUID.fromString("4E454152-434C-4950-0000-000000000002")
        val PUBLIC_KEY_HASH_UUID: UUID = UUID.fromString("4E454152-434C-4950-0000-000000000003")
        val DATA_TRANSFER_UUID: UUID = UUID.fromString("4E454152-434C-4950-0000-000000000004")
        val DATA_ACK_UUID: UUID = UUID.fromString("4E454152-434C-4950-0000-000000000005")

        // Default MTU payload size
        private const val DEFAULT_MTU = 20
    }

    // Callback interface
    interface Callback {
        fun onDeviceDiscovered(peripheralAddress: String, deviceId: String?, publicKeyHash: String?, rssi: Int)
        fun onDeviceLost(peripheralAddress: String)
        fun onDeviceConnected(peripheralAddress: String, deviceId: String)
        fun onDeviceDisconnected(peripheralAddress: String, deviceId: String?)
        fun onDataReceived(peripheralAddress: String, data: ByteArray)
        fun onAckReceived(peripheralAddress: String, data: ByteArray)
        fun onError(peripheralAddress: String?, error: String)
    }

    var callback: Callback? = null

    private val bluetoothManager: BluetoothManager? =
        context.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
    private val bluetoothAdapter: BluetoothAdapter? = bluetoothManager?.adapter

    // Central mode
    private var bleScanner: BluetoothLeScanner? = null
    private var isScanning = false
    private val peripherals = ConcurrentHashMap<String, BluetoothDevice>() // address -> device
    private val connectedGatts = ConcurrentHashMap<String, BluetoothGatt>() // address -> gatt
    private val peripheralDeviceIds = ConcurrentHashMap<String, String>() // address -> device ID
    private val mtuCache = ConcurrentHashMap<String, Int>() // address -> mtu

    // Peripheral mode
    private var gattServer: BluetoothGattServer? = null
    private var advertiser: BluetoothLeAdvertiser? = null
    private var isAdvertising = false
    private var localDeviceId: String = ""
    private var localPublicKeyHash: String = ""

    // Data transfer
    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())

    // GATT characteristics for peripheral mode
    private var deviceIdCharacteristic: BluetoothGattCharacteristic? = null
    private var publicKeyHashCharacteristic: BluetoothGattCharacteristic? = null
    private var dataTransferCharacteristic: BluetoothGattCharacteristic? = null
    private var dataAckCharacteristic: BluetoothGattCharacteristic? = null

    // Track connected centrals (devices that connected to us as peripheral)
    private val connectedCentrals = ConcurrentHashMap<String, BluetoothDevice>()
    private val centralSubscriptions = ConcurrentHashMap<String, MutableSet<UUID>>()

    // ==================== Configuration ====================

    fun configure(deviceId: String, publicKeyHash: String) {
        this.localDeviceId = deviceId
        this.localPublicKeyHash = publicKeyHash
        Log.i(TAG, "Configured with deviceId=$deviceId")
    }

    // ==================== Central Mode (Scanner) ====================

    fun startScanning() {
        Log.i(TAG, "startScanning() called, isScanning=$isScanning")
        if (isScanning) {
            Log.i(TAG, "Already scanning, skipping")
            return
        }
        if (bluetoothAdapter?.isEnabled != true) {
            Log.w(TAG, "Cannot scan - Bluetooth not enabled")
            return
        }

        bleScanner = bluetoothAdapter.bluetoothLeScanner
        if (bleScanner == null) {
            Log.w(TAG, "Cannot scan - BLE scanner not available")
            return
        }

        val settings = ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)
            .build()

        // Scan without filter first, then filter in callback
        // This is needed because macOS CoreBluetooth may not include Service UUID in advertisement packet
        Log.i(TAG, "Starting BLE scan without filter (will filter by service in callback)")
        bleScanner?.startScan(null, settings, scanCallback)
        isScanning = true
    }

    fun stopScanning() {
        if (!isScanning) return

        Log.i(TAG, "Stopping BLE scan")
        bleScanner?.stopScan(scanCallback)
        isScanning = false
    }

    fun connect(peripheralAddress: String) {
        val device = peripherals[peripheralAddress]
        if (device == null) {
            Log.w(TAG, "Peripheral not found: $peripheralAddress")
            return
        }

        if (connectedGatts.containsKey(peripheralAddress)) {
            Log.i(TAG, "Already connected to $peripheralAddress")
            return
        }

        Log.i(TAG, "Connecting to peripheral: $peripheralAddress")
        val gatt = device.connectGatt(context, false, gattCallback, BluetoothDevice.TRANSPORT_LE)
        if (gatt != null) {
            connectedGatts[peripheralAddress] = gatt
        } else {
            Log.e(TAG, "Failed to create GATT connection")
            callback?.onError(peripheralAddress, "Failed to create GATT connection")
        }
    }

    fun disconnect(peripheralAddress: String) {
        val gatt = connectedGatts[peripheralAddress]
        if (gatt != null) {
            Log.i(TAG, "Disconnecting from peripheral: $peripheralAddress")
            gatt.disconnect()
            gatt.close()
            connectedGatts.remove(peripheralAddress)
        }
    }

    fun isConnected(peripheralAddress: String): Boolean {
        return connectedGatts.containsKey(peripheralAddress) || connectedCentrals.containsKey(peripheralAddress)
    }

    fun getMtu(peripheralAddress: String): Int {
        // For centrals connected to us, try to get their preferred MTU if available
        val central = connectedCentrals[peripheralAddress]
        if (central != null) {
            // Android GATT server doesn't easily expose the negotiated MTU per central
            // We'll use the default or a cached value if we ever implement MTU callbacks for server
            return 512 // Most modern devices support this
        }
        return mtuCache[peripheralAddress] ?: DEFAULT_MTU
    }

    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            val device = result.device
            val address = device.address
            val deviceName = result.scanRecord?.deviceName

            // Check if this device advertises our service UUID
            val serviceUuids = result.scanRecord?.serviceUuids
            val hasNearClipService = serviceUuids?.any { it.uuid == SERVICE_UUID } == true

            // Also check if we already know this device (from previous discovery)
            val isKnownDevice = peripheralDeviceIds.containsKey(address)
            // Handle specific case for macOS which may not advertise service UUID but has correct name
            val hasNameMatch = deviceName?.contains("NearClip", ignoreCase = true) == true

            // Debug logging for discovered devices with service UUIDs
            if (serviceUuids?.isNotEmpty() == true) {
                Log.d(TAG, "Scan: $address name='$deviceName' services=${serviceUuids?.map { it.uuid }} hasNearClip=$hasNearClipService")
            }

            if (!hasNearClipService && !isKnownDevice && !hasNameMatch) {
                // Not a NearClip device, ignore
                return
            }

            Log.i(TAG, "NearClip device found: $address name='$deviceName' hasService=$hasNearClipService isKnown=$isKnownDevice hasName=$hasNameMatch")

            // Store peripheral reference
            peripherals[address] = device

            // Check if we already know this device's ID
            val knownDeviceId = peripheralDeviceIds[address]
            if (knownDeviceId != null) {
                // Already know this device, just notify with known info
                callback?.onDeviceDiscovered(address, knownDeviceId, null, result.rssi)
                return
            }

            // Notify delegate about discovery with unknown device ID
            // Rust layer will decide whether to connect for discovery
            callback?.onDeviceDiscovered(address, null, null, result.rssi)
        }

        override fun onScanFailed(errorCode: Int) {
            Log.e(TAG, "Scan failed with error: $errorCode")
            isScanning = false
            callback?.onError(null, "BLE scan failed: $errorCode")
        }
    }

    private val gattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            val address = gatt.device.address

            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    Log.i(TAG, "Connected to peripheral: $address")
                    // gatt is already in connectedGatts if we initiated via connect(),
                    // but make sure it's up to date
                    connectedGatts[address] = gatt
                    gatt.discoverServices()
                    gatt.requestMtu(512)
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.i(TAG, "Disconnected from peripheral: $address")
                    val deviceId = peripheralDeviceIds[address]
                    connectedGatts.remove(address)
                    peripheralDeviceIds.remove(address)
                    mtuCache.remove(address)
                    callback?.onDeviceDisconnected(address, deviceId)
                    gatt.close()
                }
            }
        }

        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            if (status != BluetoothGatt.GATT_SUCCESS) {
                Log.e(TAG, "Service discovery failed: $status")
                return
            }

            val service = gatt.getService(SERVICE_UUID)
            if (service == null) {
                Log.w(TAG, "NearClip service not found")
                gatt.disconnect()
                return
            }

            // Read device ID characteristic
            val deviceIdChar = service.getCharacteristic(DEVICE_ID_UUID)
            if (deviceIdChar != null) {
                gatt.readCharacteristic(deviceIdChar)
            }
        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            if (status != BluetoothGatt.GATT_SUCCESS) return

            val value = characteristic.value ?: return
            val address = gatt.device.address

            when (characteristic.uuid) {
                DEVICE_ID_UUID -> {
                    val deviceId = String(value, Charsets.UTF_8)
                    peripheralDeviceIds[address] = deviceId
                    Log.i(TAG, "Device ID read: $deviceId")
                    callback?.onDeviceConnected(address, deviceId)

                    // Read public key hash
                    val service = gatt.getService(SERVICE_UUID)
                    val pubKeyChar = service?.getCharacteristic(PUBLIC_KEY_HASH_UUID)
                    if (pubKeyChar != null) {
                        gatt.readCharacteristic(pubKeyChar)
                    }

                    // Subscribe to ACK notifications
                    subscribeCharacteristic(address, DATA_ACK_UUID.toString())

                    // Subscribe to Data Transfer notifications (for receiving data)
                    subscribeCharacteristic(address, DATA_TRANSFER_UUID.toString())
                }
                PUBLIC_KEY_HASH_UUID -> {
                    val hash = String(value, Charsets.UTF_8)
                    Log.i(TAG, "Public key hash read: $hash")
                }
            }
        }

        override fun onCharacteristicChanged(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic
        ) {
            val address = gatt.device.address
            when (characteristic.uuid) {
                DATA_ACK_UUID -> {
                    val value = characteristic.value
                    if (value != null) {
                        Log.i(TAG, "ACK received from $address: ${value.size} bytes")
                        // Forward ACK to callback - use device ID if available, otherwise address
                        val deviceId = peripheralDeviceIds[address] ?: address
                        callback?.onAckReceived(deviceId, value)
                    }
                }
                DATA_TRANSFER_UUID -> {
                    val value = characteristic.value
                    if (value != null) {
                        Log.d(TAG, "Data chunk received from $address: ${value.size} bytes")
                        callback?.onDataReceived(address, value)
                    }
                }
            }
        }

        override fun onMtuChanged(gatt: BluetoothGatt, mtu: Int, status: Int) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                val payloadMtu = mtu - 3
                mtuCache[gatt.device.address] = payloadMtu
                Log.i(TAG, "MTU changed to $mtu, payload size: $payloadMtu")
            }
        }
    }

    // ==================== Peripheral Mode (Advertiser) ====================

    fun startAdvertising(serviceData: ByteArray? = null) {
        if (isAdvertising) return
        if (bluetoothAdapter?.isEnabled != true) {
            Log.w(TAG, "Cannot advertise - Bluetooth not enabled")
            return
        }
        if (localDeviceId.isEmpty()) {
            Log.w(TAG, "Cannot advertise - device ID not configured")
            return
        }

        advertiser = bluetoothAdapter.bluetoothLeAdvertiser
        if (advertiser == null) {
            Log.w(TAG, "Cannot advertise - BLE advertiser not available")
            return
        }

        // Only set up GATT server once
        if (gattServer == null) {
            setupGattServer()
            if (gattServer == null) {
                Log.e(TAG, "Failed to set up GATT server, cannot advertise")
                return
            }
        }

        val settings = AdvertiseSettings.Builder()
            .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)
            .setConnectable(true)
            .setTimeout(0)
            .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_HIGH)
            .build()

        val data = AdvertiseData.Builder()
            .setIncludeDeviceName(false)
            .addServiceUuid(ParcelUuid(SERVICE_UUID))
            .apply {
                if (serviceData != null) {
                    addServiceData(ParcelUuid(SERVICE_UUID), serviceData)
                }
            }
            .build()

        Log.i(TAG, "Starting BLE advertisement")
        advertiser?.startAdvertising(settings, data, advertiseCallback)
    }

    fun stopAdvertising() {
        if (!isAdvertising) return

        Log.i(TAG, "Stopping BLE advertisement")
        advertiser?.stopAdvertising(advertiseCallback)
        gattServer?.close()
        gattServer = null
        isAdvertising = false
    }

    private fun setupGattServer() {
        // Close existing server if any
        if (gattServer != null) {
            Log.i(TAG, "Closing existing GATT server")
            gattServer?.close()
            gattServer = null
        }

        gattServer = bluetoothManager?.openGattServer(context, gattServerCallback)

        if (gattServer == null) {
            Log.e(TAG, "Failed to open GATT server")
            return
        }

        val service = BluetoothGattService(SERVICE_UUID, BluetoothGattService.SERVICE_TYPE_PRIMARY)

        deviceIdCharacteristic = BluetoothGattCharacteristic(
            DEVICE_ID_UUID,
            BluetoothGattCharacteristic.PROPERTY_READ,
            BluetoothGattCharacteristic.PERMISSION_READ
        ).apply {
            value = localDeviceId.toByteArray(Charsets.UTF_8)
        }
        service.addCharacteristic(deviceIdCharacteristic)

        publicKeyHashCharacteristic = BluetoothGattCharacteristic(
            PUBLIC_KEY_HASH_UUID,
            BluetoothGattCharacteristic.PROPERTY_READ,
            BluetoothGattCharacteristic.PERMISSION_READ
        ).apply {
            value = localPublicKeyHash.toByteArray(Charsets.UTF_8)
        }
        service.addCharacteristic(publicKeyHashCharacteristic)

        dataTransferCharacteristic = BluetoothGattCharacteristic(
            DATA_TRANSFER_UUID,
            BluetoothGattCharacteristic.PROPERTY_WRITE or BluetoothGattCharacteristic.PROPERTY_WRITE_NO_RESPONSE or BluetoothGattCharacteristic.PROPERTY_NOTIFY,
            BluetoothGattCharacteristic.PERMISSION_WRITE or BluetoothGattCharacteristic.PERMISSION_READ
        )
        // Add CCCD for notifications
        dataTransferCharacteristic?.addDescriptor(
            BluetoothGattDescriptor(
                UUID.fromString("00002902-0000-1000-8000-00805f9b34fb"),
                BluetoothGattDescriptor.PERMISSION_READ or BluetoothGattDescriptor.PERMISSION_WRITE
            )
        )
        service.addCharacteristic(dataTransferCharacteristic)

        dataAckCharacteristic = BluetoothGattCharacteristic(
            DATA_ACK_UUID,
            BluetoothGattCharacteristic.PROPERTY_READ or BluetoothGattCharacteristic.PROPERTY_NOTIFY,
            BluetoothGattCharacteristic.PERMISSION_READ
        )
        // Add CCCD for notifications
        dataAckCharacteristic?.addDescriptor(
            BluetoothGattDescriptor(
                UUID.fromString("00002902-0000-1000-8000-00805f9b34fb"),
                BluetoothGattDescriptor.PERMISSION_READ or BluetoothGattDescriptor.PERMISSION_WRITE
            )
        )
        service.addCharacteristic(dataAckCharacteristic)

        gattServer?.addService(service)
        Log.i(TAG, "GATT server configured")
    }

    private val advertiseCallback = object : AdvertiseCallback() {
        override fun onStartSuccess(settingsInEffect: AdvertiseSettings) {
            Log.i(TAG, "Advertisement started successfully")
            isAdvertising = true
        }

        override fun onStartFailure(errorCode: Int) {
            Log.e(TAG, "Advertisement failed: $errorCode")
            isAdvertising = false
            callback?.onError(null, "BLE advertise failed: $errorCode")
        }
    }

    private val gattServerCallback = object : BluetoothGattServerCallback() {
        override fun onServiceAdded(status: Int, service: BluetoothGattService) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                Log.i(TAG, "GATT service added successfully: ${service.uuid}")
            } else {
                Log.e(TAG, "Failed to add GATT service: status=$status")
            }
        }

        override fun onConnectionStateChange(device: BluetoothDevice, status: Int, newState: Int) {
            val address = device.address
            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    Log.i(TAG, "Central connected: $address")
                    connectedCentrals[address] = device
                    centralSubscriptions[address] = mutableSetOf()
                    // Delay notification to give time for service discovery and subscription
                    scope.launch {
                        delay(1000)
                        callback?.onDeviceConnected(address, address)
                    }
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.i(TAG, "Central disconnected: $address")
                    connectedCentrals.remove(address)
                    centralSubscriptions.remove(address)
                    callback?.onDeviceDisconnected(address, null)
                }
            }
        }

        override fun onCharacteristicReadRequest(
            device: BluetoothDevice,
            requestId: Int,
            offset: Int,
            characteristic: BluetoothGattCharacteristic
        ) {
            val value = when (characteristic.uuid) {
                DEVICE_ID_UUID -> localDeviceId.toByteArray(Charsets.UTF_8)
                PUBLIC_KEY_HASH_UUID -> localPublicKeyHash.toByteArray(Charsets.UTF_8)
                else -> null
            }

            if (value != null) {
                gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, value)
            } else {
                gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_FAILURE, offset, null)
            }
        }

        override fun onCharacteristicWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            characteristic: BluetoothGattCharacteristic,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            value: ByteArray?
        ) {
            Log.i(TAG, "onCharacteristicWriteRequest: uuid=${characteristic.uuid} from ${device.address}, size=${value?.size}, responseNeeded=$responseNeeded")
            if (value != null) {
                when (characteristic.uuid) {
                    DATA_TRANSFER_UUID -> {
                        // Forward chunk directly to callback
                        callback?.onDataReceived(device.address, value)
                    }
                    DATA_ACK_UUID -> {
                        // Forward ACK directly to callback
                        callback?.onAckReceived(device.address, value)
                    }
                }
            }

            if (responseNeeded) {
                gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, null)
            }
        }

        override fun onDescriptorWriteRequest(
            device: BluetoothDevice,
            requestId: Int,
            descriptor: BluetoothGattDescriptor,
            preparedWrite: Boolean,
            responseNeeded: Boolean,
            offset: Int,
            value: ByteArray?
        ) {
            // Handle CCCD writes for notification subscription
            val cccdUuid = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb")
            if (descriptor.uuid == cccdUuid) {
                if (value != null) {
                    val isNotifyEnabled = value.contentEquals(BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE)
                    val isIndicateEnabled = value.contentEquals(BluetoothGattDescriptor.ENABLE_INDICATION_VALUE)
                    val charUuid = descriptor.characteristic?.uuid
                    Log.i(TAG, "CCCD write for $charUuid: notify=$isNotifyEnabled, indicate=$isIndicateEnabled")

                    if (charUuid != null) {
                        if (isNotifyEnabled || isIndicateEnabled) {
                            centralSubscriptions[device.address]?.add(charUuid)
                        } else {
                            centralSubscriptions[device.address]?.remove(charUuid)
                        }
                    }
                }
                // Always respond with success for CCCD writes
                if (responseNeeded) {
                    gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, null)
                }
            } else {
                if (responseNeeded) {
                    gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_FAILURE, offset, null)
                }
            }
        }
    }

    private fun sendAck(device: BluetoothDevice, messageId: UShort) {
        // ACK format matches Rust: 2 bytes for message_id (LE)
        val ackData = ByteArray(2)
        val msgId = messageId.toInt()
        ackData[0] = (msgId and 0xFF).toByte()
        ackData[1] = ((msgId shr 8) and 0xFF).toByte()

        val ackChar = dataAckCharacteristic
        if (ackChar == null) {
            Log.e(TAG, "sendAck: dataAckCharacteristic is null!")
            return
        }

        val server = gattServer
        if (server == null) {
            Log.e(TAG, "sendAck: gattServer is null!")
            return
        }

        ackChar.value = ackData
        val result = server.notifyCharacteristicChanged(device, ackChar, false)
        Log.i(TAG, "sendAck: Sent ACK for messageId=$msgId to ${device.address}, result=$result")
    }

    // ==================== Data Transfer ====================

    fun writeData(peripheralAddress: String, data: ByteArray): String {
        val gatt = connectedGatts[peripheralAddress]
        if (gatt != null) {
            return sendDataToCentral(peripheralAddress, gatt, data)
        }

        val centralDevice = connectedCentrals[peripheralAddress]
        if (centralDevice != null) {
            return sendDataToPeripheral(peripheralAddress, centralDevice, data)
        }

        return "Device not connected: $peripheralAddress"
    }

    private fun sendDataToCentral(address: String, gatt: BluetoothGatt, data: ByteArray): String {
        val service = gatt.getService(SERVICE_UUID)
        val characteristic = service?.getCharacteristic(DATA_TRANSFER_UUID)
        if (characteristic == null) {
            return "Data transfer characteristic not found"
        }

        Log.i(TAG, "Sending ${data.size} bytes to central")
        characteristic.value = data
        characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
        gatt.writeCharacteristic(characteristic)

        return "" // Success
    }

    private fun sendDataToPeripheral(address: String, device: BluetoothDevice, data: ByteArray): String {
        Log.i(TAG, "Sending ${data.size} bytes via notify to peripheral")
        dataTransferCharacteristic?.value = data
        gattServer?.notifyCharacteristicChanged(device, dataTransferCharacteristic, false)

        return "" // Success
    }

    // ==================== GATT Operations ====================

    /**
     * Read a GATT characteristic value.
     * @param peripheralUuid The peripheral address or device ID
     * @param charUuid The characteristic UUID as string
     * @return The characteristic value, or empty byte array on error
     */
    fun readCharacteristic(peripheralUuid: String, charUuid: String): ByteArray {
        // Find the actual peripheral address if deviceId was passed
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == peripheralUuid }?.key ?: peripheralUuid

        val gatt = connectedGatts[peripheralAddress]
        if (gatt == null) {
            Log.w(TAG, "readCharacteristic: Device not connected: $peripheralUuid")
            return ByteArray(0)
        }

        try {
            val service = gatt.getService(SERVICE_UUID)
            if (service == null) {
                Log.w(TAG, "readCharacteristic: Service not found")
                return ByteArray(0)
            }

            val uuid = UUID.fromString(charUuid)
            val characteristic = service.getCharacteristic(uuid)
            if (characteristic == null) {
                Log.w(TAG, "readCharacteristic: Characteristic not found: $charUuid")
                return ByteArray(0)
            }

            val value = characteristic.value ?: return ByteArray(0)
            Log.i(TAG, "readCharacteristic: Read ${value.size} bytes from $charUuid")
            return value
        } catch (e: Exception) {
            Log.e(TAG, "readCharacteristic: Error reading characteristic: ${e.message}")
            return ByteArray(0)
        }
    }

    /**
     * Write to a GATT characteristic.
     * In Central mode: writes to the remote peripheral's characteristic
     * In Peripheral mode: sends notification to the connected central
     * @param peripheralUuid The peripheral address or device ID
     * @param charUuid The characteristic UUID as string
     * @param data The data to write
     * @return Empty string on success, error message on failure
     */
    fun writeCharacteristic(peripheralUuid: String, charUuid: String, data: ByteArray): String {
        // Find the actual peripheral address if deviceId was passed
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == peripheralUuid }?.key ?: peripheralUuid

        // First try Central mode (we connected to a peripheral)
        val gatt = connectedGatts[peripheralAddress]
        if (gatt != null) {
            try {
                val service = gatt.getService(SERVICE_UUID)
                if (service == null) {
                    return "Service not found"
                }

                val uuid = UUID.fromString(charUuid)
                val characteristic = service.getCharacteristic(uuid)
                if (characteristic == null) {
                    return "Characteristic not found: $charUuid"
                }

                characteristic.value = data
                characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT
                val success = gatt.writeCharacteristic(characteristic)

                if (!success) {
                    return "Failed to initiate write operation"
                }

                Log.i(TAG, "writeCharacteristic: Wrote ${data.size} bytes to $charUuid (Central mode)")
                return "" // Success
            } catch (e: Exception) {
                Log.e(TAG, "writeCharacteristic: Error: ${e.message}")
                return "Error: ${e.message}"
            }
        }

        // Try Peripheral mode (central connected to us)
        val central = connectedCentrals[peripheralAddress]
        if (central != null) {
            try {
                val server = gattServer ?: return "GATT server not initialized"
                val uuid = UUID.fromString(charUuid)

                // Find the characteristic in our GATT server
                val characteristic = when (uuid) {
                    DATA_TRANSFER_UUID -> dataTransferCharacteristic
                    DATA_ACK_UUID -> dataAckCharacteristic
                    else -> return "Characteristic not found for peripheral mode: $charUuid"
                }

                if (characteristic == null) {
                    return "Characteristic not initialized: $charUuid"
                }

                characteristic.value = data
                val success = server.notifyCharacteristicChanged(central, characteristic, false)

                if (!success) {
                    return "Failed to send notification"
                }

                Log.i(TAG, "writeCharacteristic: Notified ${data.size} bytes via $charUuid (Peripheral mode)")
                return "" // Success
            } catch (e: Exception) {
                Log.e(TAG, "writeCharacteristic (Peripheral): Error: ${e.message}")
                return "Error: ${e.message}"
            }
        }

        return "Device not connected: $peripheralUuid"
    }

    /**
     * Subscribe to notifications/indications from a GATT characteristic.
     * @param peripheralUuid The peripheral address or device ID
     * @param charUuid The characteristic UUID as string
     * @return Empty string on success, error message on failure
     */
    fun subscribeCharacteristic(peripheralUuid: String, charUuid: String): String {
        // Find the actual peripheral address if deviceId was passed
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == peripheralUuid }?.key ?: peripheralUuid

        // If we are Peripheral and a Central is connected to us, we don't need to "subscribe" to them
        // in the traditional sense, but the transport layer calls this to ensure it can receive ACKs.
        // In Peripheral mode, we "notify" the Central, so this is a no-op or just a connection check.
        if (connectedCentrals.containsKey(peripheralAddress)) {
            Log.i(TAG, "subscribeCharacteristic: Device is connected as Central, skipping subscription")
            return "" // Success
        }

        val gatt = connectedGatts[peripheralAddress]
        if (gatt == null) {
            return "Device not connected: $peripheralUuid"
        }

        try {
            val service = gatt.getService(SERVICE_UUID)
            if (service == null) {
                return "Service not found"
            }

            val uuid = UUID.fromString(charUuid)
            val characteristic = service.getCharacteristic(uuid)
            if (characteristic == null) {
                return "Characteristic not found: $charUuid"
            }

            // Enable local notifications
            val success = gatt.setCharacteristicNotification(characteristic, true)
            if (!success) {
                return "Failed to enable notifications"
            }

            // Enable notifications on the characteristic descriptor
            val cccd = characteristic.getDescriptor(UUID.fromString("00002902-0000-1000-8000-00805f9b34fb"))
            if (cccd != null) {
                // Enable notifications (0x0100)
                cccd.value = BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE
                gatt.writeDescriptor(cccd)
            }

            Log.i(TAG, "subscribeCharacteristic: Subscribed to $charUuid")
            return "" // Success
        } catch (e: Exception) {
            Log.e(TAG, "subscribeCharacteristic: Error: ${e.message}")
            return "Error: ${e.message}"
        }
    }

    // ==================== Helper Methods ====================

    /**
     * Check if a device is discovered (found via scanning).
     * @param deviceId The device ID to check (will be matched against peripheralDeviceIds)
     */
    fun isDeviceDiscovered(deviceId: String): Boolean {
        // Check if any peripheral has this device ID
        return peripheralDeviceIds.containsValue(deviceId) ||
               // Also check if the deviceId is a peripheral address directly
               peripherals.containsKey(deviceId)
    }

    /**
     * Check if a device is connected.
     * @param deviceId The device ID to check
     */
    fun isDeviceConnected(deviceId: String): Boolean {
        // Check by device ID in peripheralDeviceIds
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == deviceId }?.key
        if (peripheralAddress != null) {
            return connectedGatts.containsKey(peripheralAddress) || connectedCentrals.containsKey(peripheralAddress)
        }
        // Also check if deviceId is a peripheral address directly
        return connectedGatts.containsKey(deviceId) || connectedCentrals.containsKey(deviceId)
    }

    /**
     * Check if there are any connected devices.
     */
    fun hasConnectedDevices(): Boolean {
        return connectedGatts.isNotEmpty() || connectedCentrals.isNotEmpty()
    }

    /**
     * Send data to a device.
     * @param deviceId The device ID to send to
     * @param data The data to send
     */
    fun sendData(deviceId: String, data: ByteArray) {
        // Find peripheral address by device ID
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == deviceId }?.key ?: deviceId
        writeData(peripheralAddress, data)
    }

    /**
     * Connect to a device by device ID.
     * @param deviceId The device ID to connect to
     */
    fun connectByDeviceId(deviceId: String) {
        // Find peripheral address by device ID, or use deviceId as address
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == deviceId }?.key ?: deviceId
        connect(peripheralAddress)
    }

    // ==================== Cleanup ====================

    fun destroy() {
        stopScanning()
        stopAdvertising()
        scope.cancel()
        connectedGatts.values.forEach { it.close() }
        connectedGatts.clear()
        peripherals.clear()
        peripheralDeviceIds.clear()
        mtuCache.clear()
        connectedCentrals.clear()
    }

}
