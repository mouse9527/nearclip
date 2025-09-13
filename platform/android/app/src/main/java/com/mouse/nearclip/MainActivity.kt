package com.mouse.nearclip

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.pm.PackageManager
import android.Manifest
import android.net.ConnectivityManager
import android.net.wifi.WifiManager
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import android.util.Log

class MainActivity : AppCompatActivity() {
    private lateinit var bluetoothAdapter: BluetoothAdapter
    private lateinit var bleScannerManager: BLEScannerManager
    private lateinit var wifiDiscoveryManager: WiFiDiscoveryManager
    private lateinit var statusText: TextView
    private lateinit var scanButton: Button
    private lateinit var modeButton: Button
    
    // Discovery mode state
    private var discoveryMode = DiscoveryMode.BLE
    
    enum class DiscoveryMode {
        BLE, WIFI, BOTH
    }
    
    // Runtime permission launcher for Bluetooth permissions
    private val requestBluetoothPermissions = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val allGranted = permissions.values.all { it }
        if (allGranted) {
            updateStatus("Bluetooth permissions granted - Ready to scan")
            scanButton.isEnabled = true
        } else {
            updateStatus("Bluetooth permissions denied. Please enable in Settings.")
            scanButton.isEnabled = false
        }
    }
    
    // Runtime permission launcher for WiFi permissions
    private val requestWifiPermissions = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val allGranted = permissions.values.all { it }
        if (allGranted) {
            updateStatus("WiFi permissions granted - Ready to discover")
            scanButton.isEnabled = true
        } else {
            updateStatus("WiFi permissions denied. Please enable in Settings.")
            scanButton.isEnabled = false
        }
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        
        // Initialize UI elements
        statusText = findViewById(R.id.statusText)
        scanButton = findViewById(R.id.scanButton)
        modeButton = findViewById(R.id.modeButton)
        
        // Initialize Bluetooth
        val bluetoothManager = getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        bluetoothAdapter = bluetoothManager.adapter
        
        if (bluetoothAdapter == null) {
            statusText.text = "Bluetooth not supported on this device"
            scanButton.isEnabled = false
            return
        }
        
        // Initialize BLE scanner
        bleScannerManager = BLEScannerManager(this, bluetoothAdapter)
        
        // Initialize WiFi discovery manager
        val connectivityManager = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        wifiDiscoveryManager = WiFiDiscoveryManager(this, connectivityManager)
        
        // Check and request permissions based on discovery mode
        checkAndRequestPermissions()
        
        // Set up scan button
        scanButton.setOnClickListener {
            toggleScanning()
        }
        
        // Set up mode button
        modeButton.setOnClickListener {
            toggleDiscoveryMode()
        }
        
        // Update mode button text
        updateModeButton()
        
        updateBluetoothStatus()
    }
    
    private fun checkAndRequestPermissions() {
        val permissionsToRequest = mutableListOf<String>()
        
        // Check Bluetooth permissions for BLE mode
        if (discoveryMode == DiscoveryMode.BLE || discoveryMode == DiscoveryMode.BOTH) {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.BLUETOOTH_SCAN) 
                != PackageManager.PERMISSION_GRANTED) {
                permissionsToRequest.add(Manifest.permission.BLUETOOTH_SCAN)
            }
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.BLUETOOTH_CONNECT) 
                != PackageManager.PERMISSION_GRANTED) {
                permissionsToRequest.add(Manifest.permission.BLUETOOTH_CONNECT)
            }
        }
        
        // Check WiFi permissions for WiFi mode
        if (discoveryMode == DiscoveryMode.WIFI || discoveryMode == DiscoveryMode.BOTH) {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.ACCESS_WIFI_STATE) 
                != PackageManager.PERMISSION_GRANTED) {
                permissionsToRequest.add(Manifest.permission.ACCESS_WIFI_STATE)
            }
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.CHANGE_WIFI_STATE) 
                != PackageManager.PERMISSION_GRANTED) {
                permissionsToRequest.add(Manifest.permission.CHANGE_WIFI_STATE)
            }
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.ACCESS_NETWORK_STATE) 
                != PackageManager.PERMISSION_GRANTED) {
                permissionsToRequest.add(Manifest.permission.ACCESS_NETWORK_STATE)
            }
        }
        
        // Check location permission (required for both BLE and WiFi discovery)
        if (ContextCompat.checkSelfPermission(this, Manifest.permission.ACCESS_FINE_LOCATION) 
            != PackageManager.PERMISSION_GRANTED) {
            permissionsToRequest.add(Manifest.permission.ACCESS_FINE_LOCATION)
        }
        
        if (permissionsToRequest.isNotEmpty()) {
            updateStatus("Requesting permissions for ${discoveryMode.name} discovery...")
            scanButton.isEnabled = false
            if (discoveryMode == DiscoveryMode.BLE) {
                requestBluetoothPermissions.launch(permissionsToRequest.toTypedArray())
            } else {
                requestWifiPermissions.launch(permissionsToRequest.toTypedArray())
            }
        } else {
            updateStatus("${discoveryMode.name} permissions already granted - Ready to discover")
            scanButton.isEnabled = true
        }
    }
    
    private fun toggleScanning() {
        when (discoveryMode) {
            DiscoveryMode.BLE -> {
                if (bleScannerManager.isScanning()) {
                    stopBLEScanning()
                } else {
                    startBLEScanning()
                }
            }
            DiscoveryMode.WIFI -> {
                if (wifiDiscoveryManager.isActive()) {
                    stopWiFiDiscovery()
                } else {
                    startWiFiDiscovery()
                }
            }
            DiscoveryMode.BOTH -> {
                if (bleScannerManager.isScanning() || wifiDiscoveryManager.isActive()) {
                    stopAllDiscovery()
                } else {
                    startAllDiscovery()
                }
            }
        }
    }
    
    private fun startBLEScanning() {
        if (!bluetoothAdapter.isEnabled) {
            Toast.makeText(this, "Please enable Bluetooth", Toast.LENGTH_SHORT).show()
            return
        }
        
        CoroutineScope(Dispatchers.Main).launch {
            try {
                val result = bleScannerManager.startScan()
                if (result.isSuccess) {
                    updateStatus("BLE scanning started...")
                    scanButton.text = "Stop BLE"
                    startBLEDeviceDiscovery()
                } else {
                    updateStatus("Failed to start BLE scan: ${result.exceptionOrNull()?.message}")
                }
            } catch (e: Exception) {
                updateStatus("Error starting BLE scan: ${e.message}")
                Log.e("NearClip", "Start BLE scan error", e)
            }
        }
    }
    
    private fun stopBLEScanning() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                val result = bleScannerManager.stopScan()
                if (result.isSuccess) {
                    updateStatus("BLE scanning stopped")
                    scanButton.text = "Start BLE"
                } else {
                    updateStatus("Failed to stop BLE scan")
                }
            } catch (e: Exception) {
                updateStatus("Error stopping BLE scan: ${e.message}")
                Log.e("NearClip", "Stop BLE scan error", e)
            }
        }
    }
    
    private fun startWiFiDiscovery() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                updateStatus("WiFi discovery starting...")
                scanButton.text = "Stop WiFi"
                startWiFiDeviceDiscovery()
            } catch (e: Exception) {
                updateStatus("Error starting WiFi discovery: ${e.message}")
                Log.e("NearClip", "Start WiFi discovery error", e)
            }
        }
    }
    
    private fun stopWiFiDiscovery() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                val result = wifiDiscoveryManager.stopDiscovery()
                if (result.isSuccess) {
                    updateStatus("WiFi discovery stopped")
                    scanButton.text = "Start WiFi"
                } else {
                    updateStatus("Failed to stop WiFi discovery")
                }
            } catch (e: Exception) {
                updateStatus("Error stopping WiFi discovery: ${e.message}")
                Log.e("NearClip", "Stop WiFi discovery error", e)
            }
        }
    }
    
    private fun startAllDiscovery() {
        updateStatus("Starting all discovery methods...")
        scanButton.text = "Stop All"
        
        // Start BLE discovery
        if (bluetoothAdapter.isEnabled) {
            startBLEDeviceDiscovery()
        }
        
        // Start WiFi discovery
        startWiFiDeviceDiscovery()
    }
    
    private fun stopAllDiscovery() {
        // Stop BLE discovery
        if (::bleScannerManager.isInitialized && bleScannerManager.isScanning()) {
            stopBLEScanning()
        }
        
        // Stop WiFi discovery
        if (::wifiDiscoveryManager.isInitialized && wifiDiscoveryManager.isActive()) {
            stopWiFiDiscovery()
        }
        
        updateStatus("All discovery stopped")
        scanButton.text = "Start All"
    }
    
    private fun startBLEDeviceDiscovery() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                bleScannerManager.startScanFlow().collect { device ->
                    runOnUiThread {
                        val deviceInfo = "[BLE] Found: ${device.name} (${device.id})\nRSSI: ${device.rssi} dBm\nSignal Quality: ${String.format("%.2f", bleScannerManager.getSignalQuality(device.id) ?: 0f)}"
                        updateStatus("$deviceInfo\n\n${statusText.text}")
                        Log.i("NearClip", "BLE Discovered device: $device")
                    }
                }
            } catch (e: Exception) {
                runOnUiThread {
                    updateStatus("BLE device discovery error: ${e.message}")
                }
                Log.e("NearClip", "BLE device discovery error", e)
            }
        }
    }
    
    private fun startWiFiDeviceDiscovery() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                wifiDiscoveryManager.startDiscovery().collect { device ->
                    runOnUiThread {
                        val deviceInfo = "[WiFi] Found: ${device.name} (${device.id})\nPort: ${device.port}\nNetwork Quality: ${String.format("%.2f", wifiDiscoveryManager.getNetworkQuality(device.id) ?: 0f)}"
                        updateStatus("$deviceInfo\n\n${statusText.text}")
                        Log.i("NearClip", "WiFi Discovered device: $device")
                    }
                }
            } catch (e: Exception) {
                runOnUiThread {
                    updateStatus("WiFi device discovery error: ${e.message}")
                }
                Log.e("NearClip", "WiFi device discovery error", e)
            }
        }
    }
    
    private fun updateBluetoothStatus() {
        val status = when {
            bluetoothAdapter == null -> "Bluetooth not supported"
            !bluetoothAdapter.isEnabled -> "Bluetooth disabled"
            else -> "Bluetooth enabled - Ready to scan"
        }
        updateStatus(status)
    }
    
    private fun updateStatus(message: String) {
        statusText.text = message
        Log.i("NearClip", "Status: $message")
    }
    
    private fun toggleDiscoveryMode() {
        // Stop current scanning if active
        if (scanButton.text.contains("Stop", ignoreCase = true)) {
            toggleScanning()
        }
        
        // Switch mode
        discoveryMode = when (discoveryMode) {
            DiscoveryMode.BLE -> DiscoveryMode.WIFI
            DiscoveryMode.WIFI -> DiscoveryMode.BOTH
            DiscoveryMode.BOTH -> DiscoveryMode.BLE
        }
        
        // Update UI
        updateModeButton()
        checkAndRequestPermissions()
        
        // Update title based on mode
        val title = when (discoveryMode) {
            DiscoveryMode.BLE -> "NearClip BLE Scanner"
            DiscoveryMode.WIFI -> "NearClip WiFi Scanner"
            DiscoveryMode.BOTH -> "NearClip Hybrid Scanner"
        }
        this.title = title
        
        updateStatus("Switched to ${discoveryMode.name} mode")
    }
    
    private fun updateModeButton() {
        modeButton.text = when (discoveryMode) {
            DiscoveryMode.BLE -> "BLE Mode"
            DiscoveryMode.WIFI -> "WiFi Mode"
            DiscoveryMode.BOTH -> "Both Mode"
        }
    }
    
    override fun onDestroy() {
        super.onDestroy()
        
        // Stop BLE discovery
        if (::bleScannerManager.isInitialized && bleScannerManager.isScanning()) {
            CoroutineScope(Dispatchers.Main).launch {
                bleScannerManager.stopScan()
            }
        }
        
        // Stop WiFi discovery
        if (::wifiDiscoveryManager.isInitialized && wifiDiscoveryManager.isActive()) {
            CoroutineScope(Dispatchers.Main).launch {
                wifiDiscoveryManager.stopDiscovery()
            }
        }
    }
}