package com.nearclip.service

import android.annotation.SuppressLint
import android.bluetooth.*
import android.bluetooth.le.*
import android.content.Context
import android.os.Handler
import android.os.Looper
import android.os.ParcelUuid
import android.util.Log
import java.util.*
import java.util.concurrent.ConcurrentHashMap

/**
 * BLE Manager for NearClip Android.
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

        // Chunk header size: [messageId: 4 bytes][sequence: 2 bytes][total: 2 bytes]
        private const val CHUNK_HEADER_SIZE = 8
    }

    // Callback interface
    interface Callback {
        fun onDeviceDiscovered(deviceId: String, publicKeyHash: String?, rssi: Int)
        fun onDeviceLost(deviceId: String)
        fun onDeviceConnected(deviceId: String)
        fun onDeviceDisconnected(deviceId: String)
        fun onDataReceived(deviceId: String, data: ByteArray)
        fun onError(deviceId: String?, error: String)
    }

    var callback: Callback? = null

    private val bluetoothManager: BluetoothManager? =
        context.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
    private val bluetoothAdapter: BluetoothAdapter? = bluetoothManager?.adapter

    // Central mode
    private var bleScanner: BluetoothLeScanner? = null
    private var isScanning = false
    private val discoveredDevices = ConcurrentHashMap<String, DiscoveredDevice>()
    private val connectedGatts = ConcurrentHashMap<String, BluetoothGatt>()
    private val peripheralDeviceIds = ConcurrentHashMap<String, String>() // peripheral address -> device ID

    // Peripheral mode
    private var gattServer: BluetoothGattServer? = null
    private var advertiser: BluetoothLeAdvertiser? = null
    private var isAdvertising = false
    private var localDeviceId: String = ""
    private var localPublicKeyHash: String = ""

    // Data transfer
    private var mtu = DEFAULT_MTU
    private val dataReassemblers = ConcurrentHashMap<String, DataReassembler>()
    private val dataChunker = DataChunker()
    private val centralDeviceIds = ConcurrentHashMap<String, String>() // central address -> device ID

    // Write queue for flow control
    private val writeQueues = ConcurrentHashMap<String, ArrayDeque<ByteArray>>()
    private val isWriting = ConcurrentHashMap<String, Boolean>()
    private val writeDelayMs = 5L  // 5ms delay between chunks

    // Auto-reconnect
    private var autoReconnect = true
    private val reconnectDevices = ConcurrentHashMap<String, BluetoothDevice>() // deviceId -> device
    private val handler = Handler(Looper.getMainLooper())
    private val baseReconnectDelayMs = 1000L
    private val maxReconnectDelayMs = 30000L
    private val maxReconnectAttempts = 5
    private val reconnectAttempts = ConcurrentHashMap<String, Int>()

    // Connection health monitoring
    private var healthCheckRunnable: Runnable? = null
    private val healthCheckIntervalMs = 30000L
    private val connectionTimeoutMs = 60000L
    private val lastActivityTimes = ConcurrentHashMap<String, Long>()

    // Power optimization
    private var scanPauseRunnable: Runnable? = null
    private val scanPauseDelayMs = 60000L  // Pause scanning after 60s if connected
    private var shouldPauseScanWhenConnected = true

    // GATT characteristics for peripheral mode
    private var deviceIdCharacteristic: BluetoothGattCharacteristic? = null
    private var publicKeyHashCharacteristic: BluetoothGattCharacteristic? = null
    private var dataTransferCharacteristic: BluetoothGattCharacteristic? = null
    private var dataAckCharacteristic: BluetoothGattCharacteristic? = null

    data class DiscoveredDevice(
        val deviceId: String,
        val device: BluetoothDevice,
        var publicKeyHash: String? = null,
        var rssi: Int = 0,
        var lastSeen: Long = System.currentTimeMillis()
    )

    // ==================== Configuration ====================

    fun configure(deviceId: String, publicKeyHash: String) {
        this.localDeviceId = deviceId
        this.localPublicKeyHash = publicKeyHash
        Log.i(TAG, "Configured with deviceId=$deviceId")
    }

    // ==================== Central Mode (Scanner) ====================

    fun startScanning() {
        if (isScanning) return
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
            .setScanMode(ScanSettings.SCAN_MODE_LOW_LATENCY)  // More aggressive scanning for discovery
            .build()

        val filters = listOf(
            ScanFilter.Builder()
                .setServiceUuid(ParcelUuid(SERVICE_UUID))
                .build()
        )

        Log.i(TAG, "Starting BLE scan with service UUID filter: $SERVICE_UUID")
        bleScanner?.startScan(filters, settings, scanCallback)
        isScanning = true
    }

    fun stopScanning() {
        if (!isScanning) return

        Log.i(TAG, "Stopping BLE scan")
        bleScanner?.stopScan(scanCallback)
        isScanning = false
    }

    // Debug method to scan without filter
    fun startScanningWithoutFilter() {
        if (isScanning) return
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

        Log.i(TAG, "Starting BLE scan WITHOUT filter (debug mode)")
        bleScanner?.startScan(null, settings, debugScanCallback)
        isScanning = true
    }

    private val debugScanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            val device = result.device
            val serviceUuids = result.scanRecord?.serviceUuids
            Log.d(TAG, "DEBUG: Found device ${device.address}, name=${device.name}, serviceUuids=$serviceUuids")

            // Check if this device has our service UUID
            if (serviceUuids?.any { it.uuid == SERVICE_UUID } == true) {
                Log.i(TAG, "DEBUG: Found NearClip device! ${device.address}")
            }
        }

        override fun onScanFailed(errorCode: Int) {
            Log.e(TAG, "DEBUG: Scan failed with error: $errorCode")
        }
    }

    fun connect(deviceId: String) {
        val device = discoveredDevices[deviceId]?.device
        if (device == null) {
            Log.w(TAG, "Device not found: $deviceId")
            return
        }

        Log.i(TAG, "Connecting to device: $deviceId")
        device.connectGatt(context, false, gattCallback, BluetoothDevice.TRANSPORT_LE)
    }

    fun disconnect(deviceId: String) {
        val gatt = connectedGatts[deviceId]
        if (gatt != null) {
            Log.i(TAG, "Disconnecting from device: $deviceId")
            gatt.disconnect()
            gatt.close()
            connectedGatts.remove(deviceId)
        }
    }

    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            val device = result.device
            val address = device.address
            Log.i(TAG, "Scan result: $address, name=${device.name}, RSSI: ${result.rssi}")

            // Connect to read device info
            if (!peripheralDeviceIds.containsKey(address) && !connectedGatts.containsKey(address)) {
                Log.i(TAG, "Discovered peripheral: $address, RSSI: ${result.rssi}")
                device.connectGatt(context, false, gattCallback, BluetoothDevice.TRANSPORT_LE)
            }
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
                    Log.i(TAG, "Connected to GATT server: $address")
                    gatt.discoverServices()
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.i(TAG, "Disconnected from GATT server: $address")
                    val deviceId = peripheralDeviceIds[address]
                    if (deviceId != null) {
                        val device = connectedGatts[deviceId]?.device
                        connectedGatts.remove(deviceId)
                        peripheralDeviceIds.remove(address)
                        lastActivityTimes.remove(deviceId)
                        writeQueues.remove(deviceId)
                        isWriting.remove(deviceId)
                        callback?.onDeviceDisconnected(deviceId)

                        // Resume scanning if no devices connected
                        if (connectedGatts.isEmpty()) {
                            cancelScanPauseTimer()
                            resumeScanningIfNeeded()
                        }

                        // Schedule auto-reconnect if enabled
                        if (autoReconnect && device != null) {
                            reconnectDevices[deviceId] = device
                            scheduleReconnect(deviceId, device)
                        }
                    }
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

            // Request MTU
            gatt.requestMtu(512)
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
                    handleDeviceIdRead(deviceId, address, gatt)
                }
                PUBLIC_KEY_HASH_UUID -> {
                    val hash = String(value, Charsets.UTF_8)
                    handlePublicKeyHashRead(hash, address)
                }
            }
        }

        override fun onCharacteristicChanged(
            gatt: BluetoothGatt,
            characteristic: BluetoothGattCharacteristic
        ) {
            if (characteristic.uuid == DATA_ACK_UUID) {
                val deviceId = peripheralDeviceIds[gatt.device.address]
                Log.i(TAG, "ACK received from $deviceId")
            }
        }

        override fun onMtuChanged(gatt: BluetoothGatt, mtu: Int, status: Int) {
            if (status == BluetoothGatt.GATT_SUCCESS) {
                this@BleManager.mtu = mtu - 3  // Subtract ATT header
                Log.i(TAG, "MTU changed to $mtu, payload size: ${this@BleManager.mtu}")
            }
        }
    }

    private fun handleDeviceIdRead(deviceId: String, address: String, gatt: BluetoothGatt) {
        peripheralDeviceIds[address] = deviceId
        connectedGatts[deviceId] = gatt

        // Clear reconnect state on successful connection
        reconnectDevices.remove(deviceId)
        reconnectAttempts.remove(deviceId)

        val device = DiscoveredDevice(
            deviceId = deviceId,
            device = gatt.device,
            rssi = 0,
            lastSeen = System.currentTimeMillis()
        )
        discoveredDevices[deviceId] = device

        Log.i(TAG, "Device ID read: $deviceId")
        callback?.onDeviceDiscovered(deviceId, null, 0)
        callback?.onDeviceConnected(deviceId)

        // Schedule scan pause for power saving
        scheduleScanPause()

        // Read public key hash
        val service = gatt.getService(SERVICE_UUID)
        val pubKeyChar = service?.getCharacteristic(PUBLIC_KEY_HASH_UUID)
        if (pubKeyChar != null) {
            gatt.readCharacteristic(pubKeyChar)
        }

        // Subscribe to ACK notifications
        val ackChar = service?.getCharacteristic(DATA_ACK_UUID)
        if (ackChar != null) {
            gatt.setCharacteristicNotification(ackChar, true)
        }
    }

    private fun handlePublicKeyHashRead(hash: String, address: String) {
        val deviceId = peripheralDeviceIds[address] ?: return
        discoveredDevices[deviceId]?.publicKeyHash = hash
        Log.i(TAG, "Public key hash read for $deviceId: $hash")
    }

    // ==================== Peripheral Mode (Advertiser) ====================

    fun startAdvertising() {
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

        // Setup GATT server first
        setupGattServer()

        val settings = AdvertiseSettings.Builder()
            .setAdvertiseMode(AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY)  // Faster advertising for better discovery
            .setConnectable(true)
            .setTimeout(0)
            .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_HIGH)  // Higher power for better range
            .build()

        val data = AdvertiseData.Builder()
            .setIncludeDeviceName(false)
            .addServiceUuid(ParcelUuid(SERVICE_UUID))
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

        // Create service
        val service = BluetoothGattService(SERVICE_UUID, BluetoothGattService.SERVICE_TYPE_PRIMARY)

        // Device ID characteristic (Read)
        deviceIdCharacteristic = BluetoothGattCharacteristic(
            DEVICE_ID_UUID,
            BluetoothGattCharacteristic.PROPERTY_READ,
            BluetoothGattCharacteristic.PERMISSION_READ
        ).apply {
            value = localDeviceId.toByteArray(Charsets.UTF_8)
        }
        service.addCharacteristic(deviceIdCharacteristic)

        // Public Key Hash characteristic (Read)
        publicKeyHashCharacteristic = BluetoothGattCharacteristic(
            PUBLIC_KEY_HASH_UUID,
            BluetoothGattCharacteristic.PROPERTY_READ,
            BluetoothGattCharacteristic.PERMISSION_READ
        ).apply {
            value = localPublicKeyHash.toByteArray(Charsets.UTF_8)
        }
        service.addCharacteristic(publicKeyHashCharacteristic)

        // Data Transfer characteristic (Write Without Response)
        dataTransferCharacteristic = BluetoothGattCharacteristic(
            DATA_TRANSFER_UUID,
            BluetoothGattCharacteristic.PROPERTY_WRITE_NO_RESPONSE,
            BluetoothGattCharacteristic.PERMISSION_WRITE
        )
        service.addCharacteristic(dataTransferCharacteristic)

        // Data ACK characteristic (Read + Notify)
        dataAckCharacteristic = BluetoothGattCharacteristic(
            DATA_ACK_UUID,
            BluetoothGattCharacteristic.PROPERTY_READ or BluetoothGattCharacteristic.PROPERTY_NOTIFY,
            BluetoothGattCharacteristic.PERMISSION_READ
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

    // Track connected centrals (devices that connected to us as peripheral)
    private val connectedCentrals = ConcurrentHashMap<String, BluetoothDevice>() // address -> device
    private val centralReadDeviceId = ConcurrentHashMap<String, Boolean>() // track if central has read our device ID

    private val gattServerCallback = object : BluetoothGattServerCallback() {
        override fun onConnectionStateChange(device: BluetoothDevice, status: Int, newState: Int) {
            when (newState) {
                BluetoothProfile.STATE_CONNECTED -> {
                    Log.i(TAG, "Central connected: ${device.address}")
                    connectedCentrals[device.address] = device
                    centralReadDeviceId[device.address] = false
                    updateActivity(device.address)
                }
                BluetoothProfile.STATE_DISCONNECTED -> {
                    Log.i(TAG, "Central disconnected: ${device.address}")
                    val deviceId = centralDeviceIds[device.address]
                    connectedCentrals.remove(device.address)
                    centralReadDeviceId.remove(device.address)
                    centralDeviceIds.remove(device.address)
                    lastActivityTimes.remove(device.address)

                    // Notify callback if we knew the device ID
                    if (deviceId != null) {
                        callback?.onDeviceDisconnected(deviceId)
                    }
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
                DEVICE_ID_UUID -> {
                    Log.i(TAG, "Central ${device.address} reading our Device ID")
                    localDeviceId.toByteArray(Charsets.UTF_8)
                }
                PUBLIC_KEY_HASH_UUID -> {
                    Log.i(TAG, "Central ${device.address} reading our Public Key Hash")
                    // When central reads public key hash, it means connection is established
                    // We need to get the central's device ID - but we can't read from central
                    // So we'll use the central's address as a temporary ID until we receive data
                    if (centralReadDeviceId[device.address] != true) {
                        centralReadDeviceId[device.address] = true
                        // Use address as temporary device ID for now
                        // The real device ID will be set when we receive data with device info
                        val tempDeviceId = "BLE-${device.address}"
                        centralDeviceIds[device.address] = tempDeviceId
                        Log.i(TAG, "Central connection established, temp deviceId: $tempDeviceId")
                        callback?.onDeviceDiscovered(tempDeviceId, null, 0)
                        callback?.onDeviceConnected(tempDeviceId)
                    }
                    localPublicKeyHash.toByteArray(Charsets.UTF_8)
                }
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
            if (characteristic.uuid == DATA_TRANSFER_UUID && value != null) {
                handleIncomingData(value, device)
            }

            if (responseNeeded) {
                gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, null)
            }
        }
    }

    private fun handleIncomingData(data: ByteArray, device: BluetoothDevice) {
        val centralId = device.address

        // Parse chunk header
        val parsed = DataChunker.parseChunk(data)
        if (parsed == null) {
            Log.w(TAG, "Invalid chunk received from $centralId, size: ${data.size}")
            return
        }

        val (messageId, sequence, total, payload) = parsed
        Log.i(TAG, "Received chunk ${sequence.toInt() + 1}/${total.toInt()} (msgId: $messageId) from $centralId, payload: ${payload.size} bytes")

        // Get or create reassembler for this central
        val reassembler = dataReassemblers.getOrPut(centralId) { DataReassembler() }

        // Check for timeout and reset if needed
        if (reassembler.isTimedOut()) {
            Log.i(TAG, "Reassembler timed out for $centralId, resetting")
            reassembler.reset()
        }

        // Add chunk to reassembler
        val completeData = reassembler.addChunk(payload, sequence.toInt(), total.toInt(), messageId)
        if (completeData != null) {
            Log.i(TAG, "Complete message received from $centralId: ${completeData.size} bytes")

            // Get device ID from central mapping, or use central ID as fallback
            val deviceId = centralDeviceIds[centralId] ?: centralId

            // Notify callback
            callback?.onDataReceived(deviceId, completeData)

            // Send ACK for complete message
            sendAck(device, messageId)
        }
    }

    private fun sendAck(device: BluetoothDevice, messageId: UInt = 0u) {
        // ACK format: [messageId: 4 bytes]
        val ackData = ByteArray(4)
        val msgId = messageId.toInt()
        ackData[0] = (msgId and 0xFF).toByte()
        ackData[1] = ((msgId shr 8) and 0xFF).toByte()
        ackData[2] = ((msgId shr 16) and 0xFF).toByte()
        ackData[3] = ((msgId shr 24) and 0xFF).toByte()

        dataAckCharacteristic?.value = ackData
        gattServer?.notifyCharacteristicChanged(device, dataAckCharacteristic, false)
        Log.i(TAG, "Sent ACK for message $messageId to central")
    }

    // ==================== Data Transfer ====================

    fun sendData(deviceId: String, data: ByteArray) {
        // First check if this is a Central connection (we connected to them)
        val gatt = connectedGatts[deviceId]
        if (gatt != null) {
            sendDataToCentral(deviceId, gatt, data)
            return
        }

        // Check if this is a Peripheral connection (they connected to us)
        // Find the central's address by device ID
        val centralAddress = centralDeviceIds.entries.find { it.value == deviceId }?.key
        if (centralAddress != null) {
            val centralDevice = connectedCentrals[centralAddress]
            if (centralDevice != null) {
                sendDataToPeripheral(deviceId, centralDevice, data)
                return
            }
        }

        Log.w(TAG, "Cannot send - device not connected: $deviceId")
    }

    private fun sendDataToCentral(deviceId: String, gatt: BluetoothGatt, data: ByteArray) {
        val service = gatt.getService(SERVICE_UUID)
        val characteristic = service?.getCharacteristic(DATA_TRANSFER_UUID)
        if (characteristic == null) {
            Log.w(TAG, "Cannot send - data transfer characteristic not found")
            return
        }

        // Create chunks with headers
        val chunks = dataChunker.createChunks(data, mtu)
        Log.i(TAG, "Sending ${data.size} bytes in ${chunks.size} chunks to $deviceId (as Central)")

        // Add to write queue
        val queue = writeQueues.getOrPut(deviceId) { ArrayDeque() }
        synchronized(queue) {
            queue.addAll(chunks)
        }

        // Start processing queue if not already writing
        if (isWriting[deviceId] != true) {
            processWriteQueue(deviceId, gatt, characteristic)
        }
    }

    private fun sendDataToPeripheral(deviceId: String, device: BluetoothDevice, data: ByteArray) {
        // When we're the Peripheral, we send data via notifications on DATA_ACK characteristic
        // But DATA_ACK is meant for ACKs. We need to use a different approach.
        // Actually, for Peripheral -> Central data transfer, we should use indications/notifications
        // on a characteristic that the Central has subscribed to.

        // For now, we'll use the DATA_TRANSFER characteristic with notifications
        // But this requires the Central to subscribe to it first.

        val chunks = dataChunker.createChunks(data, mtu)
        Log.i(TAG, "Sending ${data.size} bytes in ${chunks.size} chunks to $deviceId (as Peripheral via notify)")

        // Send each chunk via notification
        handler.post {
            for (chunk in chunks) {
                dataTransferCharacteristic?.value = chunk
                gattServer?.notifyCharacteristicChanged(device, dataTransferCharacteristic, false)

                // Small delay between chunks
                Thread.sleep(writeDelayMs)
            }
            Log.i(TAG, "Finished sending ${chunks.size} chunks to $deviceId")
        }
    }

    private fun processWriteQueue(deviceId: String, gatt: BluetoothGatt, characteristic: BluetoothGattCharacteristic) {
        val queue = writeQueues[deviceId] ?: return

        val chunk: ByteArray?
        synchronized(queue) {
            chunk = queue.pollFirst()
        }

        if (chunk == null) {
            isWriting[deviceId] = false
            Log.i(TAG, "Write queue empty for $deviceId")
            return
        }

        isWriting[deviceId] = true

        // Write chunk
        characteristic.value = chunk
        characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
        gatt.writeCharacteristic(characteristic)

        // Update activity time
        updateActivity(deviceId)

        // Schedule next write with delay for flow control
        handler.postDelayed({
            processWriteQueue(deviceId, gatt, characteristic)
        }, writeDelayMs)
    }

    fun setMtu(newMtu: Int) {
        this.mtu = maxOf(20, newMtu - 3)
        Log.i(TAG, "MTU set to $mtu")
    }

    fun isDeviceConnected(deviceId: String): Boolean {
        // Check if connected as Central (we connected to them)
        if (connectedGatts.containsKey(deviceId)) return true
        // Check if connected as Peripheral (they connected to us)
        return centralDeviceIds.containsValue(deviceId)
    }

    fun hasConnectedDevices(): Boolean {
        // Check if we have any BLE connections (either as Central or Peripheral)
        return connectedGatts.isNotEmpty() || centralDeviceIds.isNotEmpty()
    }

    fun isDeviceDiscovered(deviceId: String): Boolean {
        return discoveredDevices.containsKey(deviceId)
    }

    fun getDiscoveredDeviceIds(): Set<String> {
        return discoveredDevices.keys.toSet()
    }

    // ==================== Auto-Reconnect ====================

    private fun scheduleReconnect(deviceId: String, device: BluetoothDevice) {
        val attempts = reconnectAttempts.getOrDefault(deviceId, 0)
        if (attempts >= maxReconnectAttempts) {
            Log.w(TAG, "Max reconnect attempts reached for $deviceId, giving up")
            reconnectDevices.remove(deviceId)
            reconnectAttempts.remove(deviceId)
            return
        }

        val delay = calculateReconnectDelay(attempts)
        Log.i(TAG, "Scheduling reconnect for $deviceId in ${delay}ms (attempt ${attempts + 1}/$maxReconnectAttempts)")

        handler.postDelayed({
            attemptReconnect(deviceId, device)
        }, delay)
    }

    private fun calculateReconnectDelay(attempts: Int): Long {
        val delay = baseReconnectDelayMs * (1L shl attempts)  // Exponential backoff
        return minOf(delay, maxReconnectDelayMs)
    }

    private fun attemptReconnect(deviceId: String, device: BluetoothDevice) {
        if (!autoReconnect) return
        if (connectedGatts.containsKey(deviceId)) {
            // Already reconnected
            reconnectDevices.remove(deviceId)
            reconnectAttempts.remove(deviceId)
            return
        }
        if (bluetoothAdapter?.isEnabled != true) {
            Log.w(TAG, "Cannot reconnect - Bluetooth not enabled")
            return
        }

        val attempts = reconnectAttempts.getOrDefault(deviceId, 0) + 1
        reconnectAttempts[deviceId] = attempts

        Log.i(TAG, "Attempting to reconnect to $deviceId (attempt $attempts/$maxReconnectAttempts)")
        device.connectGatt(context, false, gattCallback, BluetoothDevice.TRANSPORT_LE)
    }

    fun setAutoReconnect(enabled: Boolean) {
        autoReconnect = enabled
        if (!enabled) {
            // Clear pending reconnects
            reconnectDevices.clear()
            reconnectAttempts.clear()
        }
    }

    // ==================== Connection Health Monitoring ====================

    fun startConnectionHealthMonitoring() {
        stopConnectionHealthMonitoring()

        healthCheckRunnable = object : Runnable {
            override fun run() {
                checkConnectionHealth()
                handler.postDelayed(this, healthCheckIntervalMs)
            }
        }
        handler.postDelayed(healthCheckRunnable!!, healthCheckIntervalMs)
        Log.i(TAG, "Connection health monitoring started")
    }

    fun stopConnectionHealthMonitoring() {
        healthCheckRunnable?.let { handler.removeCallbacks(it) }
        healthCheckRunnable = null
    }

    private fun checkConnectionHealth() {
        val now = System.currentTimeMillis()

        for ((deviceId, gatt) in connectedGatts) {
            val lastActivity = lastActivityTimes[deviceId] ?: now

            if (now - lastActivity > connectionTimeoutMs) {
                Log.w(TAG, "Connection timeout for device $deviceId, disconnecting")

                // Force disconnect and trigger reconnection
                gatt.disconnect()
            }
        }
    }

    fun updateActivity(deviceId: String) {
        lastActivityTimes[deviceId] = System.currentTimeMillis()
    }

    fun resetReconnectAttempts(deviceId: String) {
        reconnectAttempts.remove(deviceId)
    }

    // ==================== Power Optimization ====================

    fun setScanPauseWhenConnected(enabled: Boolean) {
        shouldPauseScanWhenConnected = enabled
        if (!enabled) {
            cancelScanPauseTimer()
        }
    }

    private fun scheduleScanPause() {
        if (!shouldPauseScanWhenConnected) return
        if (connectedGatts.isEmpty()) return

        cancelScanPauseTimer()

        scanPauseRunnable = Runnable {
            pauseScanningForPowerSaving()
        }
        handler.postDelayed(scanPauseRunnable!!, scanPauseDelayMs)
    }

    private fun cancelScanPauseTimer() {
        scanPauseRunnable?.let { handler.removeCallbacks(it) }
        scanPauseRunnable = null
    }

    private fun pauseScanningForPowerSaving() {
        if (!isScanning || connectedGatts.isEmpty()) return

        Log.i(TAG, "Pausing scan for power saving (connected devices: ${connectedGatts.size})")
        bleScanner?.stopScan(scanCallback)
        // Keep isScanning conceptually true so we resume when a device disconnects
        isScanning = false
    }

    private fun resumeScanningIfNeeded() {
        if (bluetoothAdapter?.isEnabled != true) return
        if (isScanning) return

        Log.i(TAG, "Resuming scan after device disconnect")
        bleScanner = bluetoothAdapter.bluetoothLeScanner
        if (bleScanner == null) return

        val settings = ScanSettings.Builder()
            .setScanMode(ScanSettings.SCAN_MODE_LOW_POWER)
            .build()

        val filters = listOf(
            ScanFilter.Builder()
                .setServiceUuid(ParcelUuid(SERVICE_UUID))
                .build()
        )

        bleScanner?.startScan(filters, settings, scanCallback)
        isScanning = true
    }

    // ==================== Cleanup ====================

    fun destroy() {
        stopScanning()
        stopAdvertising()
        stopConnectionHealthMonitoring()
        cancelScanPauseTimer()
        // Clear reconnect state
        autoReconnect = false
        handler.removeCallbacksAndMessages(null)
        reconnectDevices.clear()
        reconnectAttempts.clear()
        lastActivityTimes.clear()
        writeQueues.clear()
        isWriting.clear()
        // Close connections
        connectedGatts.values.forEach { it.close() }
        connectedGatts.clear()
        discoveredDevices.clear()
        peripheralDeviceIds.clear()
    }

    // ==================== Data Reassembler ====================

    class DataReassembler {
        private val chunks = mutableMapOf<Int, ByteArray>()
        private var totalChunks = 0
        private var messageId: UInt = 0u
        private var lastActivityTime = System.currentTimeMillis()
        private val timeoutMs = 30000L  // 30 seconds

        fun isTimedOut(): Boolean {
            return System.currentTimeMillis() - lastActivityTime > timeoutMs
        }

        fun reset() {
            chunks.clear()
            totalChunks = 0
            messageId = 0u
        }

        fun addChunk(data: ByteArray, sequence: Int, total: Int, msgId: UInt): ByteArray? {
            lastActivityTime = System.currentTimeMillis()

            if (this.messageId != msgId) {
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
        private var messageIdCounter: UInt = 0u

        /**
         * Create chunks from data with headers
         * Format: [messageId: 4 bytes][sequence: 2 bytes][total: 2 bytes][payload]
         */
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

                // Build chunk with header
                val chunk = ByteArray(CHUNK_HEADER_SIZE + payload.size)

                // Message ID (4 bytes, little endian)
                val msgId = messageId.toInt()
                chunk[0] = (msgId and 0xFF).toByte()
                chunk[1] = ((msgId shr 8) and 0xFF).toByte()
                chunk[2] = ((msgId shr 16) and 0xFF).toByte()
                chunk[3] = ((msgId shr 24) and 0xFF).toByte()

                // Sequence number (2 bytes, little endian)
                val seq = sequence.toInt()
                chunk[4] = (seq and 0xFF).toByte()
                chunk[5] = ((seq shr 8) and 0xFF).toByte()

                // Total chunks (2 bytes, little endian)
                chunk[6] = (totalChunks and 0xFF).toByte()
                chunk[7] = ((totalChunks shr 8) and 0xFF).toByte()

                // Payload
                payload.copyInto(chunk, CHUNK_HEADER_SIZE)

                chunks.add(chunk)
                offset += chunkPayloadSize
                sequence++
            }

            // Handle empty data case
            if (chunks.isEmpty()) {
                val chunk = ByteArray(CHUNK_HEADER_SIZE)
                val msgId = messageId.toInt()
                chunk[0] = (msgId and 0xFF).toByte()
                chunk[1] = ((msgId shr 8) and 0xFF).toByte()
                chunk[2] = ((msgId shr 16) and 0xFF).toByte()
                chunk[3] = ((msgId shr 24) and 0xFF).toByte()
                chunk[4] = 0
                chunk[5] = 0
                chunk[6] = 1
                chunk[7] = 0
                chunks.add(chunk)
            }

            return chunks
        }

        companion object {
            /**
             * Parse a chunk and extract header information
             * Returns: (messageId, sequence, total, payload) or null if invalid
             */
            fun parseChunk(data: ByteArray): ChunkInfo? {
                if (data.size < CHUNK_HEADER_SIZE) return null

                val messageId = ((data[0].toInt() and 0xFF) or
                        ((data[1].toInt() and 0xFF) shl 8) or
                        ((data[2].toInt() and 0xFF) shl 16) or
                        ((data[3].toInt() and 0xFF) shl 24)).toUInt()

                val sequence = ((data[4].toInt() and 0xFF) or
                        ((data[5].toInt() and 0xFF) shl 8)).toUShort()

                val total = ((data[6].toInt() and 0xFF) or
                        ((data[7].toInt() and 0xFF) shl 8)).toUShort()

                val payload = data.copyOfRange(CHUNK_HEADER_SIZE, data.size)

                return ChunkInfo(messageId, sequence, total, payload)
            }
        }
    }

    data class ChunkInfo(
        val messageId: UInt,
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
