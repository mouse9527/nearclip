package com.nearclip.ui.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.nearclip.ConnectionManager
import com.nearclip.SettingsViewModel
import com.nearclip.ffi.FfiDeviceInfo

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(
    onNavigateBack: () -> Unit,
    connectionManager: ConnectionManager = viewModel(),
    settingsViewModel: SettingsViewModel = viewModel()
) {
    val settings by settingsViewModel.settings.collectAsState()
    val pairedDevices by connectionManager.pairedDevices.collectAsState()
    val pausedDeviceIds by connectionManager.pausedDeviceIds.collectAsState()

    var showDeleteDialog by remember { mutableStateOf<FfiDeviceInfo?>(null) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Settings") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { paddingValues ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
        ) {
            // Sync Settings
            item {
                SettingsSection(title = "Sync")
            }

            item {
                SettingsSwitch(
                    title = "WiFi Sync",
                    subtitle = "Sync over local network",
                    icon = Icons.Default.Wifi,
                    checked = settings.wifiEnabled,
                    onCheckedChange = { settingsViewModel.setWifiEnabled(it) }
                )
            }

            item {
                SettingsSwitch(
                    title = "Bluetooth Sync",
                    subtitle = "Sync via Bluetooth Low Energy",
                    icon = Icons.Default.Bluetooth,
                    checked = settings.bleEnabled,
                    onCheckedChange = { settingsViewModel.setBleEnabled(it) }
                )
            }

            item {
                SettingsSwitch(
                    title = "Auto Connect",
                    subtitle = "Automatically connect to paired devices",
                    icon = Icons.Default.SyncAlt,
                    checked = settings.autoConnect,
                    onCheckedChange = { settingsViewModel.setAutoConnect(it) }
                )
            }

            item {
                SettingsSwitch(
                    title = "Sync Notifications",
                    subtitle = "Show notification when clipboard syncs",
                    icon = Icons.Default.Notifications,
                    checked = settings.syncNotifications,
                    onCheckedChange = { settingsViewModel.setSyncNotifications(it) }
                )
            }

            item {
                HorizontalDivider(modifier = Modifier.padding(vertical = 8.dp))
            }

            // Paired Devices Section
            item {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "Paired Devices",
                        style = MaterialTheme.typography.titleSmall,
                        color = MaterialTheme.colorScheme.primary
                    )
                    Text(
                        text = "${pairedDevices.size}/${ConnectionManager.MAX_PAIRED_DEVICES}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }

            if (pairedDevices.isEmpty()) {
                item {
                    Text(
                        text = "No paired devices",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)
                    )
                }
            } else {
                items(pairedDevices) { device ->
                    PairedDeviceItem(
                        device = device,
                        isPaused = pausedDeviceIds.contains(device.id),
                        onDelete = { showDeleteDialog = device },
                        onTogglePause = {
                            if (pausedDeviceIds.contains(device.id)) {
                                connectionManager.resumeDevice(device.id)
                            } else {
                                connectionManager.pauseDevice(device.id)
                            }
                        }
                    )
                }
            }

            item {
                HorizontalDivider(modifier = Modifier.padding(vertical = 8.dp))
            }

            // About Section
            item {
                SettingsSection(title = "About")
            }

            item {
                SettingsItem(
                    title = "Version",
                    subtitle = "1.0.0",
                    icon = Icons.Default.Info
                )
            }

            item {
                SettingsItem(
                    title = "Open Source Licenses",
                    icon = Icons.Default.Description,
                    onClick = { /* TODO: Show licenses */ }
                )
            }
        }
    }

    // Delete confirmation dialog
    showDeleteDialog?.let { device ->
        AlertDialog(
            onDismissRequest = { showDeleteDialog = null },
            title = { Text("Remove Device") },
            text = { Text("Are you sure you want to remove \"${device.name}\"? You will need to pair again to sync with this device.") },
            confirmButton = {
                TextButton(
                    onClick = {
                        connectionManager.removeDevice(device.id)
                        showDeleteDialog = null
                    },
                    colors = ButtonDefaults.textButtonColors(
                        contentColor = MaterialTheme.colorScheme.error
                    )
                ) {
                    Text("Remove")
                }
            },
            dismissButton = {
                TextButton(onClick = { showDeleteDialog = null }) {
                    Text("Cancel")
                }
            }
        )
    }
}

@Composable
fun PairedDeviceItem(
    device: FfiDeviceInfo,
    isPaused: Boolean = false,
    onDelete: () -> Unit,
    onTogglePause: () -> Unit = {}
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 12.dp)
            .then(if (isPaused) Modifier.alpha(0.6f) else Modifier),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = when (device.platform.name) {
                "MAC_OS" -> Icons.Default.Laptop
                "ANDROID" -> Icons.Default.PhoneAndroid
                else -> Icons.Default.Devices
            },
            contentDescription = null,
            tint = if (isPaused)
                MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.5f)
            else
                MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.width(16.dp))
        Column(modifier = Modifier.weight(1f)) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(
                    text = device.name,
                    style = MaterialTheme.typography.bodyLarge,
                    color = if (isPaused)
                        MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f)
                    else
                        MaterialTheme.colorScheme.onSurface
                )
                if (isPaused) {
                    Spacer(modifier = Modifier.width(8.dp))
                    Text(
                        text = "(Paused)",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.tertiary
                    )
                }
            }
            Text(
                text = device.platform.name.lowercase().replaceFirstChar { it.uppercase() },
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        // Pause/Resume button
        IconButton(onClick = onTogglePause) {
            Icon(
                imageVector = if (isPaused) Icons.Default.PlayArrow else Icons.Default.Pause,
                contentDescription = if (isPaused) "Resume sync" else "Pause sync",
                tint = if (isPaused)
                    MaterialTheme.colorScheme.primary
                else
                    MaterialTheme.colorScheme.tertiary
            )
        }
        // Delete button
        IconButton(onClick = onDelete) {
            Icon(
                imageVector = Icons.Default.Delete,
                contentDescription = "Remove device",
                tint = MaterialTheme.colorScheme.error
            )
        }
    }
}

@Composable
fun SettingsSection(title: String) {
    Text(
        text = title,
        style = MaterialTheme.typography.titleSmall,
        color = MaterialTheme.colorScheme.primary,
        modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp)
    )
}

@Composable
fun SettingsSwitch(
    title: String,
    subtitle: String? = null,
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onCheckedChange(!checked) }
            .padding(horizontal = 16.dp, vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.width(16.dp))
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = title,
                style = MaterialTheme.typography.bodyLarge
            )
            subtitle?.let {
                Text(
                    text = it,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange
        )
    }
}

@Composable
fun SettingsItem(
    title: String,
    subtitle: String? = null,
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    onClick: (() -> Unit)? = null
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .then(
                if (onClick != null) Modifier.clickable(onClick = onClick)
                else Modifier
            )
            .padding(horizontal = 16.dp, vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.width(16.dp))
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = title,
                style = MaterialTheme.typography.bodyLarge
            )
            subtitle?.let {
                Text(
                    text = it,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        }
        if (onClick != null) {
            Icon(
                imageVector = Icons.Default.ChevronRight,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}
