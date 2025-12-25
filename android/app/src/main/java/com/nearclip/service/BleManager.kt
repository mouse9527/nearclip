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

        // Chunk header size: [messageId: 4 bytes][sequence: 2 bytes][total: 2 bytes]
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
    private val handler = Handler(Looper.getMainLooper())

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

        val filters = listOf(
            ScanFilter.Builder()
                .setServiceUuid(ParcelUuid(SERVICE_UUID))
                .build()
        )

        Log.i(TAG, "Starting BLE scan")
        bleScanner?.startScan(filters, settings, scanCallback)
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

        Log.i(TAG, "Connecting to peripheral: $peripheralAddress")
        device.connectGatt(context, false, gattCallback, BluetoothDevice.TRANSPORT_LE)
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

    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult) {
            val device = result.device
            val address = device.address

            // Store peripheral reference
            peripherals[address] = device

            Log.i(TAG, "Discovered peripheral: $address, RSSI: ${result.rssi}")

            // Notify callback - Rust layer will decide whether to connect
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
                    val ackChar = service?.getCharacteristic(DATA_ACK_UUID)
                    if (ackChar != null) {
                        gatt.setCharacteristicNotification(ackChar, true)
                    }
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
            if (characteristic.uuid == DATA_ACK_UUID) {
                Log.i(TAG, "ACK received")
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
            BluetoothGattCharacteristic.PROPERTY_WRITE_NO_RESPONSE,
            BluetoothGattCharacteristic.PERMISSION_WRITE
        )
        service.addCharacteristic(dataTransferCharacteristic)

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
            if (characteristic.uuid == DATA_TRANSFER_UUID && value != null) {
                handleIncomingData(value, device)
            }

            if (responseNeeded) {
                gattServer?.sendResponse(device, requestId, BluetoothGatt.GATT_SUCCESS, offset, null)
            }
        }
    }

    private fun handleIncomingData(data: ByteArray, device: BluetoothDevice) {
        val address = device.address

        val parsed = DataChunker.parseChunk(data)
        if (parsed == null) {
            Log.w(TAG, "Invalid chunk received from $address")
            return
        }

        val (messageId, sequence, total, payload) = parsed

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

    private fun sendAck(device: BluetoothDevice, messageId: UInt) {
        val ackData = ByteArray(4)
        val msgId = messageId.toInt()
        ackData[0] = (msgId and 0xFF).toByte()
        ackData[1] = ((msgId shr 8) and 0xFF).toByte()
        ackData[2] = ((msgId shr 16) and 0xFF).toByte()
        ackData[3] = ((msgId shr 24) and 0xFF).toByte()

        dataAckCharacteristic?.value = ackData
        gattServer?.notifyCharacteristicChanged(device, dataAckCharacteristic, false)
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

        for (chunk in chunks) {
            characteristic.value = chunk
            characteristic.writeType = BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
            gatt.writeCharacteristic(characteristic)
            Thread.sleep(5) // Small delay between chunks
        }

        return "" // Success
    }

    private fun sendDataToPeripheral(address: String, device: BluetoothDevice, data: ByteArray): String {
        val mtu = mtuCache[address] ?: DEFAULT_MTU
        val chunks = dataChunker.createChunks(data, mtu)
        Log.i(TAG, "Sending ${data.size} bytes in ${chunks.size} chunks via notify")

        handler.post {
            for (chunk in chunks) {
                dataTransferCharacteristic?.value = chunk
                gattServer?.notifyCharacteristicChanged(device, dataTransferCharacteristic, false)
                Thread.sleep(5)
            }
        }

        return "" // Success
    }

    // ==================== Cleanup ====================

    fun destroy() {
        stopScanning()
        stopAdvertising()
        handler.removeCallbacksAndMessages(null)
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
        private var messageId: UInt = 0u
        private var lastActivityTime = System.currentTimeMillis()
        private val timeoutMs = 30000L

        fun isTimedOut(): Boolean = System.currentTimeMillis() - lastActivityTime > timeoutMs

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
                chunk[0] = (msgId and 0xFF).toByte()
                chunk[1] = ((msgId shr 8) and 0xFF).toByte()
                chunk[2] = ((msgId shr 16) and 0xFF).toByte()
                chunk[3] = ((msgId shr 24) and 0xFF).toByte()

                val seq = sequence.toInt()
                chunk[4] = (seq and 0xFF).toByte()
                chunk[5] = ((seq shr 8) and 0xFF).toByte()

                chunk[6] = (totalChunks and 0xFF).toByte()
                chunk[7] = ((totalChunks shr 8) and 0xFF).toByte()

                payload.copyInto(chunk, CHUNK_HEADER_SIZE)

                chunks.add(chunk)
                offset += chunkPayloadSize
                sequence++
            }

            if (chunks.isEmpty()) {
                val chunk = ByteArray(CHUNK_HEADER_SIZE)
                val msgId = messageId.toInt()
                chunk[0] = (msgId and 0xFF).toByte()
                chunk[1] = ((msgId shr 8) and 0xFF).toByte()
                chunk[2] = ((msgId shr 16) and 0xFF).toByte()
                chunk[3] = ((msgId shr 24) and 0xFF).toByte()
                chunk[6] = 1
                chunks.add(chunk)
            }

            return chunks
        }

        companion object {
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
