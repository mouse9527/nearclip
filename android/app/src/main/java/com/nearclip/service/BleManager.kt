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

        // Chunk header size (Rust format): [messageId: 2 bytes][sequence: 2 bytes][total: 2 bytes][payloadLength: 2 bytes]
        private const val CHUNK_HEADER_SIZE = 8
    }

    // Callback interface
    interface Callback {
        fun onDeviceDiscovered(peripheralAddress: String, deviceId: String?, publicKeyHash: String?, rssi: Int)
        fun onDeviceLost(peripheralAddress: String)
        fun onDeviceConnected(peripheralAddress: String, deviceId: String)
        fun onDeviceDisconnected(peripheralAddress: String, deviceId: String?)
        fun onDataReceived(peripheralAddress: String, data: ByteArray)
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
    private val dataReassemblers = ConcurrentHashMap<String, DataReassembler>()
    private val dataChunker = DataChunker()
    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())

    // GATT characteristics for peripheral mode
    private var deviceIdCharacteristic: BluetoothGattCharacteristic? = null
    private var publicKeyHashCharacteristic: BluetoothGattCharacteristic? = null
    private var dataTransferCharacteristic: BluetoothGattCharacteristic? = null
    private var dataAckCharacteristic: BluetoothGattCharacteristic? = null

    // Track connected centrals (devices that connected to us as peripheral)
    private val connectedCentrals = ConcurrentHashMap<String, BluetoothDevice>()

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
        return mtuCache[peripheralAddress] ?: DEFAULT_MTU
    }

    // Track devices we're currently connecting to for discovery
    private val pendingDiscoveryConnections = ConcurrentHashMap<String, Boolean>()

    // Throttle discovery connections - track last connection attempt time
    private val lastDiscoveryAttempt = ConcurrentHashMap<String, Long>()
    private val discoveryThrottleMs = 30_000L // 30 seconds between discovery attempts for same device
    private val maxConcurrentDiscovery = 2 // Max concurrent discovery connections

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

            // Check if we're already connecting to this device
            if (pendingDiscoveryConnections.containsKey(address)) {
                return
            }

            // Check if already connected
            if (connectedGatts.containsKey(address)) {
                return
            }

            // Throttle: check if we recently tried to connect to this device
            val now = System.currentTimeMillis()
            val lastAttempt = lastDiscoveryAttempt[address] ?: 0L
            if (now - lastAttempt < discoveryThrottleMs) {
                return
            }

            // Limit concurrent discovery connections to prevent connection storm
            if (pendingDiscoveryConnections.size >= maxConcurrentDiscovery) {
                return
            }

            // Auto-connect to read device ID (for discovery purposes)
            Log.i(TAG, "Auto-connecting to $address to read device ID (pending: ${pendingDiscoveryConnections.size})")
            pendingDiscoveryConnections[address] = true
            lastDiscoveryAttempt[address] = now
            scope.launch {
                try {
                    device.connectGatt(context, false, discoveryGattCallback, BluetoothDevice.TRANSPORT_LE)
                } catch (e: Exception) {
                    Log.e(TAG, "Failed to connect for discovery: ${e.message}")
                    pendingDiscoveryConnections.remove(address)
                }
            }
        }

        override fun onScanFailed(errorCode: Int) {
            Log.e(TAG, "Scan failed with error: $errorCode")
            isScanning = false
            callback?.onError(null, "BLE scan failed: $errorCode")
        }
    }

    // Separate GATT callback for discovery connections (read device ID then disconnect)
    private val discoveryGattCallback = object : BluetoothGattCallback() {
        override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
            val address = gatt.device.address

            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    Log.i(TAG, "Discovery: Connected to $address")
                    gatt.discoverServices()
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.i(TAG, "Discovery: Disconnected from $address")
                    pendingDiscoveryConnections.remove(address)
                    gatt.close()
                }
            }
        }

        override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
            val address = gatt.device.address
            if (status != BluetoothGatt.GATT_SUCCESS) {
                Log.e(TAG, "Discovery: Service discovery failed for $address: $status")
                gatt.disconnect()
                return
            }

            val service = gatt.getService(SERVICE_UUID)
            if (service == null) {
                Log.w(TAG, "Discovery: NearClip service not found on $address")
                gatt.disconnect()
                return
            }

            // Read device ID characteristic
            val deviceIdChar = service.getCharacteristic(DEVICE_ID_UUID)
            if (deviceIdChar != null) {
                gatt.readCharacteristic(deviceIdChar)
            } else {
                Log.w(TAG, "Discovery: Device ID characteristic not found on $address")
                gatt.disconnect()
            }
        }

        override fun onCharacteristicRead(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic,
            status: Int
        ) {
            val address = gatt.device.address
            if (status != BluetoothGatt.GATT_SUCCESS) {
                Log.e(TAG, "Discovery: Failed to read characteristic from $address")
                gatt.disconnect()
                return
            }

            val value = characteristic.value ?: run {
                gatt.disconnect()
                return
            }

            when (characteristic.uuid) {
                DEVICE_ID_UUID -> {
                    val deviceId = String(value, Charsets.UTF_8)
                    peripheralDeviceIds[address] = deviceId
                    Log.i(TAG, "Discovery: Got device ID from $address: $deviceId")

                    // Notify callback with device ID
                    callback?.onDeviceDiscovered(address, deviceId, null, 0)

                    // Disconnect after reading device ID (discovery complete)
                    gatt.disconnect()
                }
                else -> {
                    gatt.disconnect()
                }
            }
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
            when (characteristic.uuid) {
                DATA_ACK_UUID -> Log.i(TAG, "ACK received")
                DATA_TRANSFER_UUID -> {
                    val value = characteristic.value
                    if (value != null) {
                        handleIncomingData(value, gatt.device)
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

        setupGattServer()

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
                // Add service data if provided
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
        gattServer = bluetoothManager?.openGattServer(context, gattServerCallback)

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
        override fun onConnectionStateChange(device: BluetoothDevice, status: Int, newState: Int) {
            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    Log.i(TAG, "Central connected: ${device.address}")
                    connectedCentrals[device.address] = device
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.i(TAG, "Central disconnected: ${device.address}")
                    connectedCentrals.remove(device.address)
                    callback?.onDeviceDisconnected(device.address, null)
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
            if (characteristic.uuid == DATA_TRANSFER_UUID && value != null) {
                handleIncomingData(value, device)
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
                    Log.i(TAG, "CCCD write for ${charUuid}: notify=$isNotifyEnabled, indicate=$isIndicateEnabled")
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

    private fun handleIncomingData(data: ByteArray, device: BluetoothDevice) {
        val address = device.address
        Log.d(TAG, "handleIncomingData: ${data.size} bytes from $address")

        val parsed = DataChunker.parseChunk(data)
        if (parsed == null) {
            Log.w(TAG, "Invalid chunk received from $address")
            return
        }

        val (messageId, sequence, total, payload) = parsed
        Log.d(TAG, "Chunk parsed: msgId=$messageId, seq=$sequence, total=$total, payload=${payload.size} bytes")

        val reassembler = dataReassemblers.getOrPut(address) { DataReassembler() }

        if (reassembler.isTimedOut()) {
            reassembler.reset()
        }

        val completeData = reassembler.addChunk(payload, sequence.toInt(), total.toInt(), messageId)
        if (completeData != null) {
            Log.i(TAG, "Complete message received: ${completeData.size} bytes")
            callback?.onDataReceived(address, completeData)
            sendAck(device, messageId)
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

        val mtu = mtuCache[address] ?: DEFAULT_MTU
        val chunks = dataChunker.createChunks(data, mtu)
        Log.i(TAG, "Sending ${data.size} bytes in ${chunks.size} chunks")

        scope.launch {
            for (chunk in chunks) {
                characteristic.value = chunk
                characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
                gatt.writeCharacteristic(characteristic)
                delay(5) // Small delay between chunks
            }
        }

        return "" // Success
    }

    private fun sendDataToPeripheral(address: String, device: BluetoothDevice, data: ByteArray): String {
        val mtu = mtuCache[address] ?: DEFAULT_MTU
        val chunks = dataChunker.createChunks(data, mtu)
        Log.i(TAG, "Sending ${data.size} bytes in ${chunks.size} chunks via notify")

        scope.launch {
            for (chunk in chunks) {
                dataTransferCharacteristic?.value = chunk
                gattServer?.notifyCharacteristicChanged(device, dataTransferCharacteristic, false)
                delay(5)
            }
        }

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
     * @param peripheralUuid The peripheral address or device ID
     * @param charUuid The characteristic UUID as string
     * @param data The data to write
     * @return Empty string on success, error message on failure
     */
    fun writeCharacteristic(peripheralUuid: String, charUuid: String, data: ByteArray): String {
        // Find the actual peripheral address if deviceId was passed
        val peripheralAddress = peripheralDeviceIds.entries.find { it.value == peripheralUuid }?.key ?: peripheralUuid

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

            characteristic.value = data
            characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_DEFAULT
            val success = gatt.writeCharacteristic(characteristic)

            if (!success) {
                return "Failed to initiate write operation"
            }

            Log.i(TAG, "writeCharacteristic: Wrote ${data.size} bytes to $charUuid")
            return "" // Success
        } catch (e: Exception) {
            Log.e(TAG, "writeCharacteristic: Error: ${e.message}")
            return "Error: ${e.message}"
        }
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

    // ==================== Data Reassembler ====================

    class DataReassembler {
        private val chunks = mutableMapOf<Int, ByteArray>()
        private var totalChunks = 0
        private var messageId: UShort = 0u
        private var lastActivityTime = System.currentTimeMillis()
        private val timeoutMs = 30000L

        fun isTimedOut(): Boolean = System.currentTimeMillis() - lastActivityTime > timeoutMs

        fun reset() {
            chunks.clear()
            totalChunks = 0
            messageId = 0u
        }

        fun addChunk(data: ByteArray, sequence: Int, total: Int, msgId: UShort): ByteArray? {
            lastActivityTime = System.currentTimeMillis()

            // Reset if different message OR first chunk of a new session
            if (this.messageId != msgId || chunks.isEmpty()) {
                chunks.clear()
                this.messageId = msgId
                this.totalChunks = total
            }

            chunks[sequence] = data

            if (chunks.size == totalChunks) {
                val result = ByteArray(chunks.values.sumOf { it.size })
                var offset = 0
                for (i in 0 until totalChunks) {
                    val chunk = chunks[i] ?: continue
                    chunk.copyInto(result, offset)
                    offset += chunk.size
                }
                chunks.clear()
                return result
            }

            return null
        }
    }

    // ==================== Data Chunker ====================

    class DataChunker {
        private var messageIdCounter: UShort = 0u

        fun createChunks(data: ByteArray, maxPayloadSize: Int): List<ByteArray> {
            val payloadSize = maxOf(1, maxPayloadSize - CHUNK_HEADER_SIZE)
            val chunks = mutableListOf<ByteArray>()

            val totalChunks = (data.size + payloadSize - 1) / payloadSize
            messageIdCounter++
            val messageId = messageIdCounter

            var offset = 0
            var sequence: UShort = 0u

            while (offset < data.size) {
                val chunkPayloadSize = minOf(payloadSize, data.size - offset)
                val payload = data.copyOfRange(offset, offset + chunkPayloadSize)

                val chunk = ByteArray(CHUNK_HEADER_SIZE + payload.size)
                val msgId = messageId.toInt()
                // message_id: 2 bytes (LE)
                chunk[0] = (msgId and 0xFF).toByte()
                chunk[1] = ((msgId shr 8) and 0xFF).toByte()

                // sequence: 2 bytes (LE)
                val seq = sequence.toInt()
                chunk[2] = (seq and 0xFF).toByte()
                chunk[3] = ((seq shr 8) and 0xFF).toByte()

                // total_chunks: 2 bytes (LE)
                chunk[4] = (totalChunks and 0xFF).toByte()
                chunk[5] = ((totalChunks shr 8) and 0xFF).toByte()

                // payload_length: 2 bytes (LE)
                chunk[6] = (chunkPayloadSize and 0xFF).toByte()
                chunk[7] = ((chunkPayloadSize shr 8) and 0xFF).toByte()

                payload.copyInto(chunk, CHUNK_HEADER_SIZE)

                chunks.add(chunk)
                offset += chunkPayloadSize
                sequence++
            }

            if (chunks.isEmpty()) {
                // Empty message - send header only with 0 payload length
                val chunk = ByteArray(CHUNK_HEADER_SIZE)
                val msgId = messageId.toInt()
                chunk[0] = (msgId and 0xFF).toByte()
                chunk[1] = ((msgId shr 8) and 0xFF).toByte()
                // sequence = 0, total = 1, payload_length = 0
                chunk[4] = 1  // total_chunks = 1
                chunks.add(chunk)
            }

            return chunks
        }

        companion object {
            fun parseChunk(data: ByteArray): ChunkInfo? {
                if (data.size < CHUNK_HEADER_SIZE) return null

                // message_id: 2 bytes (LE)
                val messageId = ((data[0].toInt() and 0xFF) or
                        ((data[1].toInt() and 0xFF) shl 8)).toUShort()

                // sequence: 2 bytes (LE)
                val sequence = ((data[2].toInt() and 0xFF) or
                        ((data[3].toInt() and 0xFF) shl 8)).toUShort()

                // total_chunks: 2 bytes (LE)
                val total = ((data[4].toInt() and 0xFF) or
                        ((data[5].toInt() and 0xFF) shl 8)).toUShort()

                // payload_length: 2 bytes (LE)
                val payloadLength = ((data[6].toInt() and 0xFF) or
                        ((data[7].toInt() and 0xFF) shl 8)).toUShort()

                val payload = data.copyOfRange(CHUNK_HEADER_SIZE, data.size)

                // Validate payload length
                if (payload.size != payloadLength.toInt()) {
                    Log.w(TAG, "Payload length mismatch: header=$payloadLength, actual=${payload.size}")
                }

                return ChunkInfo(messageId, sequence, total, payload)
            }
        }
    }

    data class ChunkInfo(
        val messageId: UShort,
        val sequence: UShort,
        val total: UShort,
        val payload: ByteArray
    ) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false
            other as ChunkInfo
            return messageId == other.messageId && sequence == other.sequence && total == other.total && payload.contentEquals(other.payload)
        }

        override fun hashCode(): Int {
            var result = messageId.hashCode()
            result = 31 * result + sequence.hashCode()
            result = 31 * result + total.hashCode()
            result = 31 * result + payload.contentHashCode()
            return result
        }
    }
}
