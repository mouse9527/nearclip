package com.example.nearclip

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Context
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import android.util.Log

class MainActivity : AppCompatActivity() {
    private lateinit var bluetoothAdapter: BluetoothAdapter
    private lateinit var bleScannerManager: BLEScannerManager
    private lateinit var statusText: TextView
    private lateinit var scanButton: Button
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        
        // Initialize UI elements
        statusText = findViewById(R.id.statusText)
        scanButton = findViewById(R.id.scanButton)
        
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
        
        // Set up scan button
        scanButton.setOnClickListener {
            toggleScanning()
        }
        
        updateBluetoothStatus()
    }
    
    private fun toggleScanning() {
        if (bleScannerManager.isScanning()) {
            stopScanning()
        } else {
            startScanning()
        }
    }
    
    private fun startScanning() {
        if (!bluetoothAdapter.isEnabled) {
            Toast.makeText(this, "Please enable Bluetooth", Toast.LENGTH_SHORT).show()
            return
        }
        
        CoroutineScope(Dispatchers.Main).launch {
            try {
                val result = bleScannerManager.startScan()
                if (result.isSuccess) {
                    updateStatus("Scanning started...")
                    scanButton.text = "Stop Scan"
                    startDeviceDiscovery()
                } else {
                    updateStatus("Failed to start scan: ${result.exceptionOrNull()?.message}")
                }
            } catch (e: Exception) {
                updateStatus("Error starting scan: ${e.message}")
                Log.e("NearClip", "Start scan error", e)
            }
        }
    }
    
    private fun stopScanning() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                val result = bleScannerManager.stopScan()
                if (result.isSuccess) {
                    updateStatus("Scanning stopped")
                    scanButton.text = "Start Scan"
                } else {
                    updateStatus("Failed to stop scan")
                }
            } catch (e: Exception) {
                updateStatus("Error stopping scan: ${e.message}")
                Log.e("NearClip", "Stop scan error", e)
            }
        }
    }
    
    private fun startDeviceDiscovery() {
        CoroutineScope(Dispatchers.Main).launch {
            try {
                bleScannerManager.startScanFlow().collect { device ->
                    runOnUiThread {
                        val deviceInfo = "Found: ${device.name} (${device.id})\nRSSI: ${device.rssi} dBm\nSignal Quality: ${String.format("%.2f", bleScannerManager.getSignalQuality(device.id) ?: 0f)}"
                        updateStatus("$deviceInfo\n\n${statusText.text}")
                        Log.i("NearClip", "Discovered device: $device")
                    }
                }
            } catch (e: Exception) {
                runOnUiThread {
                    updateStatus("Device discovery error: ${e.message}")
                }
                Log.e("NearClip", "Device discovery error", e)
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
    
    override fun onDestroy() {
        super.onDestroy()
        if (::bleScannerManager.isInitialized && bleScannerManager.isScanning()) {
            CoroutineScope(Dispatchers.Main).launch {
                bleScannerManager.stopScan()
            }
        }
    }
}