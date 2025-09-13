package com.mouse.nearclip

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import android.widget.Toast
import android.content.pm.PackageManager
import androidx.core.content.ContextCompat
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.launch

class ComposeMainActivity : ComponentActivity() {
    private lateinit var unifiedDiscoveryManager: UnifiedDiscoveryManager
    
    // Runtime permission launcher
    private val requestPermissions = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val allGranted = permissions.values.all { it }
        if (allGranted) {
            startDiscovery()
        } else {
            // Handle permission denial
            println("Permissions denied - cannot start discovery")
        }
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Initialize managers
        val bluetoothAdapter = android.bluetooth.BluetoothAdapter.getDefaultAdapter()
        val connectivityManager = getSystemService(CONNECTIVITY_SERVICE) as android.net.ConnectivityManager
        
        unifiedDiscoveryManager = UnifiedDiscoveryManager(this, bluetoothAdapter, connectivityManager)
        
        setContent {
            NearClipTheme {
                UnifiedDiscoveryApp(
                    onDeviceSelected = { device ->
                        handleDeviceSelected(device)
                    },
                    onRefresh = {
                        refreshDiscovery()
                    }
                )
            }
        }
        
        // Start observing discovery state
        observeDiscoveryState()
        
        // Check and request permissions
        checkAndRequestPermissions()
    }
    
    @Composable
    fun NearClipTheme(
        content: @Composable () -> Unit
    ) {
        MaterialTheme(
            colorScheme = lightColorScheme(),
            typography = Typography(),
            content = content
        )
    }
    
    @Composable
    fun UnifiedDiscoveryApp(
        onDeviceSelected: (DiscoveredDevice) -> Unit,
        onRefresh: () -> Unit
    ) {
        val discoveryState = remember { mutableStateOf(UnifiedDiscoveryState()) }
        
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            UnifiedDeviceDiscoveryScreen(
                discoveryState = discoveryState.value,
                onDeviceSelected = onDeviceSelected,
                onRefresh = onRefresh
            )
        }
    }
    
    private fun observeDiscoveryState() {
        lifecycleScope.launch {
            // Update UI based on discovery manager state
            launch {
                unifiedDiscoveryManager.discoveredDevices.collectLatest { devices ->
                    // TODO: Update UI state with new devices
                    println("Discovered devices updated: ${devices.size} devices")
                }
            }
            
            launch {
                unifiedDiscoveryManager.isScanning.collectLatest { isScanning ->
                    // TODO: Update UI state with scanning status
                    println("Scanning status: $isScanning")
                }
            }
            
            launch {
                unifiedDiscoveryManager.currentStrategy.collectLatest { strategy ->
                    // TODO: Update UI state with current strategy
                    println("Discovery strategy: $strategy")
                }
            }
        }
    }
    
    private fun checkAndRequestPermissions() {
        val permissionsToRequest = mutableListOf<String>()
        
        // Check Bluetooth permissions
        if (ContextCompat.checkSelfPermission(
                this, 
                android.Manifest.permission.BLUETOOTH_SCAN
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            permissionsToRequest.add(android.Manifest.permission.BLUETOOTH_SCAN)
        }
        
        if (ContextCompat.checkSelfPermission(
                this, 
                android.Manifest.permission.BLUETOOTH_CONNECT
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            permissionsToRequest.add(android.Manifest.permission.BLUETOOTH_CONNECT)
        }
        
        // Check WiFi permissions
        if (ContextCompat.checkSelfPermission(
                this, 
                android.Manifest.permission.ACCESS_WIFI_STATE
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            permissionsToRequest.add(android.Manifest.permission.ACCESS_WIFI_STATE)
        }
        
        if (ContextCompat.checkSelfPermission(
                this, 
                android.Manifest.permission.CHANGE_WIFI_STATE
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            permissionsToRequest.add(android.Manifest.permission.CHANGE_WIFI_STATE)
        }
        
        // Check location permission
        if (ContextCompat.checkSelfPermission(
                this, 
                android.Manifest.permission.ACCESS_FINE_LOCATION
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            permissionsToRequest.add(android.Manifest.permission.ACCESS_FINE_LOCATION)
        }
        
        if (permissionsToRequest.isNotEmpty()) {
            requestPermissions.launch(permissionsToRequest.toTypedArray())
        } else {
            startDiscovery()
        }
    }
    
    private fun startDiscovery() {
        lifecycleScope.launch {
            try {
                unifiedDiscoveryManager.startDiscovery()
            } catch (e: Exception) {
                // Handle discovery start error
                println("Failed to start discovery: ${e.message}")
            }
        }
    }
    
    private fun handleDeviceSelected(device: DiscoveredDevice) {
        // Handle device selection (e.g., initiate connection)
        println("Selected device: ${device.name} (${device.id})")
        
        // Show a toast or dialog for device selection
        Toast.makeText(
            this,
            "已选择设备: ${device.name}",
            Toast.LENGTH_SHORT
        ).show()
    }
    
    private fun refreshDiscovery() {
        unifiedDiscoveryManager.refreshDevices()
    }
    
    override fun onDestroy() {
        super.onDestroy()
        
        // Clean up discovery resources
        lifecycleScope.launch {
            unifiedDiscoveryManager.stopDiscovery()
        }
    }
}