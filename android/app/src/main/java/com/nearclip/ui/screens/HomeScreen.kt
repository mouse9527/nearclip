package com.nearclip.ui.screens

import android.content.Context
import androidx.compose.animation.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.viewmodel.compose.viewModel
import com.nearclip.ConnectionManager
import com.nearclip.LocalNearClipService
import com.nearclip.ffi.DeviceStatus
import com.nearclip.service.NearClipService
import java.text.SimpleDateFormat
import java.util.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen(
    onNavigateToPairing: () -> Unit,
    onNavigateToSettings: () -> Unit,
    connectionManager: ConnectionManager = viewModel()
) {
    val context = LocalContext.current
    val service = LocalNearClipService.current
    val lifecycleOwner = LocalLifecycleOwner.current
    val isRunning by connectionManager.isRunning.collectAsState()
    val pairedDevices by connectionManager.pairedDevices.collectAsState()
    val connectedDevices by connectionManager.connectedDevices.collectAsState()
    val lastReceivedClipboard by connectionManager.lastReceivedClipboard.collectAsState()
    val lastError by connectionManager.lastError.collectAsState()

    // Refresh devices when screen becomes visible (e.g., returning from PairingScreen)
    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            if (event == Lifecycle.Event.ON_RESUME) {
                // Refresh from service if available, otherwise from connectionManager
                if (service != null) {
                    connectionManager.refreshFromService(service)
                } else {
                    connectionManager.refreshDevices()
                }
            }
        }
        lifecycleOwner.lifecycle.addObserver(observer)
        onDispose {
            lifecycleOwner.lifecycle.removeObserver(observer)
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("NearClip") },
                actions = {
                    IconButton(onClick = onNavigateToSettings) {
                        Icon(Icons.Default.Settings, contentDescription = "Settings")
                    }
                }
            )
        },
        floatingActionButton = {
            FloatingActionButton(onClick = onNavigateToPairing) {
                Icon(Icons.Default.Add, contentDescription = "Add Device")
            }
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .padding(16.dp)
        ) {
            // Status Card
            StatusCard(
                isRunning = isRunning,
                connectedCount = connectedDevices.size,
                onToggle = {
                    if (isRunning) {
                        NearClipService.stopService(context)
                        connectionManager.stop()
                    } else {
                        NearClipService.startService(context)
                        connectionManager.start()
                    }
                },
                onSyncNow = {
                    NearClipService.syncNow(context)
                }
            )

            Spacer(modifier = Modifier.height(16.dp))

            // Last received clipboard
            AnimatedVisibility(
                visible = lastReceivedClipboard != null,
                enter = fadeIn() + expandVertically(),
                exit = fadeOut() + shrinkVertically()
            ) {
                lastReceivedClipboard?.let { (content, fromDevice) ->
                    LastSyncCard(
                        content = content.toString(Charsets.UTF_8),
                        fromDevice = fromDevice
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                }
            }

            // Error message
            AnimatedVisibility(
                visible = lastError != null,
                enter = fadeIn() + expandVertically(),
                exit = fadeOut() + shrinkVertically()
            ) {
                lastError?.let { error ->
                    ErrorCard(message = error)
                    Spacer(modifier = Modifier.height(16.dp))
                }
            }

            // Devices Section
            Text(
                text = "Paired Devices",
                style = MaterialTheme.typography.titleMedium
            )

            Spacer(modifier = Modifier.height(8.dp))

            if (pairedDevices.isEmpty()) {
                EmptyDevicesCard(onAddDevice = onNavigateToPairing)
            } else {
                LazyColumn {
                    items(pairedDevices) { device ->
                        DeviceCard(
                            name = device.name,
                            platform = device.platform.name,
                            status = device.status,
                            onConnect = {
                                // Use service's manager for connection to ensure sync uses same manager
                                service?.connectDevice(device.id)
                                // Delay refresh since connect is async
                                android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
                                    connectionManager.refreshFromService(service)
                                }, 500)
                            },
                            onDisconnect = {
                                service?.disconnectDevice(device.id)
                                // Delay refresh since disconnect is async
                                android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
                                    connectionManager.refreshFromService(service)
                                }, 500)
                            }
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                    }
                }
            }
        }
    }
}

@Composable
fun StatusCard(
    isRunning: Boolean,
    connectedCount: Int,
    onToggle: () -> Unit,
    onSyncNow: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = if (isRunning)
                MaterialTheme.colorScheme.primaryContainer
            else
                MaterialTheme.colorScheme.surfaceVariant
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Column {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Icon(
                            imageVector = if (isRunning) Icons.Default.Cloud else Icons.Default.CloudOff,
                            contentDescription = null,
                            tint = if (isRunning)
                                MaterialTheme.colorScheme.primary
                            else
                                MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text(
                            text = if (isRunning) "Running" else "Stopped",
                            style = MaterialTheme.typography.titleMedium
                        )
                    }
                    if (isRunning && connectedCount > 0) {
                        Text(
                            text = "$connectedCount device(s) connected",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }

                Switch(
                    checked = isRunning,
                    onCheckedChange = { onToggle() }
                )
            }

            // Sync Now button when running
            AnimatedVisibility(
                visible = isRunning,
                enter = fadeIn() + expandVertically(),
                exit = fadeOut() + shrinkVertically()
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(top = 12.dp),
                    horizontalArrangement = Arrangement.End
                ) {
                    FilledTonalButton(onClick = onSyncNow) {
                        Icon(
                            Icons.Default.Sync,
                            contentDescription = null,
                            modifier = Modifier.size(18.dp)
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text("Sync Now")
                    }
                }
            }
        }
    }
}

@Composable
fun LastSyncCard(
    content: String,
    fromDevice: String
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.secondaryContainer
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Default.ContentPaste,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.secondary
            )
            Spacer(modifier = Modifier.width(12.dp))
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = "Received from $fromDevice",
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSecondaryContainer
                )
                Text(
                    text = content.take(100) + if (content.length > 100) "..." else "",
                    style = MaterialTheme.typography.bodySmall,
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
}

@Composable
fun ErrorCard(message: String) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.errorContainer
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Default.Error,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.error
            )
            Spacer(modifier = Modifier.width(12.dp))
            Text(
                text = message,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onErrorContainer
            )
        }
    }
}

@Composable
fun DeviceCard(
    name: String,
    platform: String,
    status: DeviceStatus,
    onConnect: () -> Unit,
    onDisconnect: () -> Unit
) {
    Card(
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Icon(
                    imageVector = when (platform) {
                        "MAC_OS" -> Icons.Default.Laptop
                        "ANDROID" -> Icons.Default.PhoneAndroid
                        else -> Icons.Default.Devices
                    },
                    contentDescription = null
                )
                Spacer(modifier = Modifier.width(12.dp))
                Column {
                    Text(
                        text = name,
                        style = MaterialTheme.typography.bodyLarge
                    )
                    Text(
                        text = status.name.lowercase().replaceFirstChar { it.uppercase() },
                        style = MaterialTheme.typography.bodySmall,
                        color = when (status) {
                            DeviceStatus.CONNECTED -> MaterialTheme.colorScheme.primary
                            DeviceStatus.CONNECTING -> MaterialTheme.colorScheme.tertiary
                            DeviceStatus.FAILED -> MaterialTheme.colorScheme.error
                            else -> MaterialTheme.colorScheme.onSurfaceVariant
                        }
                    )
                }
            }

            when (status) {
                DeviceStatus.CONNECTED -> {
                    TextButton(onClick = onDisconnect) {
                        Text("Disconnect")
                    }
                }
                DeviceStatus.CONNECTING -> {
                    CircularProgressIndicator(
                        modifier = Modifier.size(24.dp),
                        strokeWidth = 2.dp
                    )
                }
                else -> {
                    TextButton(onClick = onConnect) {
                        Text("Connect")
                    }
                }
            }
        }
    }
}

@Composable
fun EmptyDevicesCard(onAddDevice: () -> Unit) {
    Card(
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(32.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Icon(
                imageVector = Icons.Default.DevicesOther,
                contentDescription = null,
                modifier = Modifier.size(48.dp),
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
            Spacer(modifier = Modifier.height(16.dp))
            Text(
                text = "No paired devices",
                style = MaterialTheme.typography.bodyLarge
            )
            Text(
                text = "Add a device to start syncing clipboards",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            Spacer(modifier = Modifier.height(16.dp))
            Button(onClick = onAddDevice) {
                Icon(Icons.Default.Add, contentDescription = null)
                Spacer(modifier = Modifier.width(8.dp))
                Text("Add Device")
            }
        }
    }
}
