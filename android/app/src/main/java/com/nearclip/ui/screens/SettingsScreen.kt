package com.nearclip.ui.screens

import android.os.Build
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.material3.Divider
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.viewmodel.compose.viewModel
import com.nearclip.ConnectionManager
import com.nearclip.LocalNearClipService
import com.nearclip.SettingsViewModel
import com.nearclip.data.SyncRetryStrategy
import com.nearclip.ffi.FfiDeviceInfo
import com.nearclip.service.NearClipAccessibilityService

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(
    onNavigateBack: () -> Unit,
    connectionManager: ConnectionManager = viewModel(),
    settingsViewModel: SettingsViewModel = viewModel()
) {
    val context = LocalContext.current
    val service = LocalNearClipService.current
    val lifecycleOwner = LocalLifecycleOwner.current
    val settings by settingsViewModel.settings.collectAsState()
    val pairedDevices by connectionManager.pairedDevices.collectAsState()
    val pausedDeviceIds by connectionManager.pausedDeviceIds.collectAsState()

    // Check accessibility service status (recheck on resume)
    var accessibilityEnabled by remember { mutableStateOf(NearClipAccessibilityService.isEnabled(context)) }

    // Refresh devices from service to ensure data consistency
    LaunchedEffect(service) {
        if (service != null) {
            connectionManager.refreshFromService(service)
        }
    }

    // Refresh when screen becomes visible
    DisposableEffect(lifecycleOwner) {
        val observer = LifecycleEventObserver { _, event ->
            if (event == Lifecycle.Event.ON_RESUME) {
                accessibilityEnabled = NearClipAccessibilityService.isEnabled(context)
                if (service != null) {
                    connectionManager.refreshFromService(service)
                }
            }
        }
        lifecycleOwner.lifecycle.addObserver(observer)
        onDispose {
            lifecycleOwner.lifecycle.removeObserver(observer)
        }
    }

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

            // Accessibility Service Section (Android 10+)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                item {
                    Divider(modifier = Modifier.padding(vertical = 8.dp))
                }

                item {
                    SettingsSection(title = "Background Clipboard Access")
                }

                item {
                    AccessibilityServiceItem(
                        isEnabled = accessibilityEnabled,
                        onEnableClick = {
                            NearClipAccessibilityService.openAccessibilitySettings(context)
                        },
                        onRefreshClick = {
                            accessibilityEnabled = NearClipAccessibilityService.isEnabled(context)
                        }
                    )
                }
            }

            item {
                Divider(modifier = Modifier.padding(vertical = 8.dp))
            }

            // Retry Strategy Section
            item {
                SettingsSection(title = "On Sync Failure")
            }

            item {
                RetryStrategySelector(
                    selectedStrategy = settings.defaultRetryStrategy,
                    onStrategySelected = { settingsViewModel.setDefaultRetryStrategy(it) }
                )
            }

            item {
                Divider(modifier = Modifier.padding(vertical = 8.dp))
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
                Divider(modifier = Modifier.padding(vertical = 8.dp))
            }

            // Debug Section
            item {
                SettingsSection(title = "Debug")
            }

            item {
                DebugSection(
                    service = service,
                    connectionManager = connectionManager
                )
            }

            item {
                Divider(modifier = Modifier.padding(vertical = 8.dp))
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
                        // Use service's unpairDevice instead of connectionManager's
                        // to ensure we use the correct FFI manager instance
                        service?.unpairDevice(device.id)
                        // Refresh device list from service
                        connectionManager.refreshFromService(service)
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

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun RetryStrategySelector(
    selectedStrategy: SyncRetryStrategy,
    onStrategySelected: (SyncRetryStrategy) -> Unit
) {
    var expanded by remember { mutableStateOf(false) }

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = Icons.Default.Refresh,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.width(16.dp))
        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = "Default Action",
                style = MaterialTheme.typography.bodyLarge
            )
            Text(
                text = selectedStrategy.description,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        ExposedDropdownMenuBox(
            expanded = expanded,
            onExpandedChange = { expanded = it }
        ) {
            OutlinedButton(
                onClick = { expanded = true },
                modifier = Modifier.menuAnchor()
            ) {
                Text(selectedStrategy.displayName)
                Icon(
                    imageVector = if (expanded) Icons.Default.ArrowDropUp else Icons.Default.ArrowDropDown,
                    contentDescription = null
                )
            }
            ExposedDropdownMenu(
                expanded = expanded,
                onDismissRequest = { expanded = false }
            ) {
                SyncRetryStrategy.entries.forEach { strategy ->
                    DropdownMenuItem(
                        text = {
                            Column {
                                Text(strategy.displayName)
                                Text(
                                    text = strategy.description,
                                    style = MaterialTheme.typography.bodySmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant
                                )
                            }
                        },
                        onClick = {
                            onStrategySelected(strategy)
                            expanded = false
                        },
                        leadingIcon = {
                            if (strategy == selectedStrategy) {
                                Icon(
                                    imageVector = Icons.Default.Check,
                                    contentDescription = null,
                                    tint = MaterialTheme.colorScheme.primary
                                )
                            }
                        }
                    )
                }
            }
        }
    }
}

@Composable
fun AccessibilityServiceItem(
    isEnabled: Boolean,
    onEnableClick: () -> Unit,
    onRefreshClick: () -> Unit
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Default.Accessibility,
                contentDescription = null,
                tint = if (isEnabled)
                    MaterialTheme.colorScheme.primary
                else
                    MaterialTheme.colorScheme.onSurfaceVariant
            )
            Spacer(modifier = Modifier.width(16.dp))
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = "Accessibility Service",
                    style = MaterialTheme.typography.bodyLarge
                )
                Text(
                    text = if (isEnabled) "Enabled" else "Required for background sync",
                    style = MaterialTheme.typography.bodySmall,
                    color = if (isEnabled)
                        MaterialTheme.colorScheme.primary
                    else
                        MaterialTheme.colorScheme.onSurfaceVariant
                )
            }

            if (isEnabled) {
                Icon(
                    imageVector = Icons.Default.CheckCircle,
                    contentDescription = "Enabled",
                    tint = MaterialTheme.colorScheme.primary
                )
            } else {
                Row {
                    IconButton(onClick = onRefreshClick) {
                        Icon(
                            imageVector = Icons.Default.Refresh,
                            contentDescription = "Refresh status",
                            tint = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                    Button(
                        onClick = onEnableClick,
                        colors = ButtonDefaults.buttonColors(
                            containerColor = MaterialTheme.colorScheme.primary
                        )
                    ) {
                        Text("Enable")
                    }
                }
            }
        }

        if (!isEnabled) {
            Spacer(modifier = Modifier.height(8.dp))
            Surface(
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(8.dp),
                color = MaterialTheme.colorScheme.secondaryContainer
            ) {
                Text(
                    text = "Android 10+ requires accessibility service to read clipboard in background. " +
                            "Find \"NearClip\" in the list and enable it.",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSecondaryContainer,
                    modifier = Modifier.padding(12.dp)
                )
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DebugSection(
    service: com.nearclip.service.NearClipService?,
    connectionManager: ConnectionManager
) {
    var testMessage by remember { mutableStateOf("Hello from NearClip!") }
    var sendStatus by remember { mutableStateOf("") }
    var selectedChannel by remember { mutableStateOf("Auto") }
    val channels = listOf("Auto", "WiFi", "BLE")

    val connectedDevices by connectionManager.connectedDevices.collectAsState()

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp)
    ) {
        // Connection Status
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier.padding(bottom = 8.dp)
        ) {
            Icon(
                imageVector = if (connectedDevices.isNotEmpty()) Icons.Default.CheckCircle else Icons.Default.Cancel,
                contentDescription = null,
                tint = if (connectedDevices.isNotEmpty()) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.error,
                modifier = Modifier.size(16.dp)
            )
            Spacer(modifier = Modifier.width(8.dp))
            Text(
                text = if (connectedDevices.isNotEmpty()) "${connectedDevices.size} device(s) connected" else "No devices connected",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        // Connected devices list
        connectedDevices.forEach { device ->
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.padding(start = 24.dp, bottom = 4.dp)
            ) {
                Icon(
                    imageVector = when (device.platform.name) {
                        "MAC_OS" -> Icons.Default.Laptop
                        else -> Icons.Default.PhoneAndroid
                    },
                    contentDescription = null,
                    tint = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.size(14.dp)
                )
                Spacer(modifier = Modifier.width(8.dp))
                Text(
                    text = device.name,
                    style = MaterialTheme.typography.bodySmall
                )
                Spacer(modifier = Modifier.width(8.dp))
                val isBleConnected = service?.isDeviceConnectedViaBle(device.id) == true
                Text(
                    text = if (isBleConnected) "BLE" else "WiFi",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.primary
                )
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        // Test message input
        OutlinedTextField(
            value = testMessage,
            onValueChange = { testMessage = it },
            label = { Text("Test Message") },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true
        )

        Spacer(modifier = Modifier.height(8.dp))

        // Channel selector
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            channels.forEach { channel ->
                FilterChip(
                    selected = selectedChannel == channel,
                    onClick = { selectedChannel = channel },
                    label = { Text(channel) }
                )
            }
        }

        Spacer(modifier = Modifier.height(8.dp))

        // Send button
        Button(
            onClick = {
                val data = testMessage.toByteArray(Charsets.UTF_8)
                when (selectedChannel) {
                    "Auto" -> {
                        service?.syncClipboard(data)
                        sendStatus = "✅ Sent via Auto channel"
                    }
                    "WiFi" -> {
                        try {
                            service?.getManager()?.syncClipboard(data)
                            sendStatus = "✅ Sent via WiFi"
                        } catch (e: Exception) {
                            sendStatus = "❌ WiFi failed: ${e.message}"
                        }
                    }
                    "BLE" -> {
                        val bleDevices = connectedDevices.filter {
                            service?.isDeviceConnectedViaBle(it.id) == true
                        }
                        if (bleDevices.isEmpty()) {
                            sendStatus = "❌ No BLE connected devices"
                        } else {
                            bleDevices.forEach { device ->
                                service?.syncClipboardViaBle(data, device.id)
                            }
                            sendStatus = "✅ Sent via BLE to ${bleDevices.size} device(s)"
                        }
                    }
                }
            },
            enabled = testMessage.isNotEmpty() && connectedDevices.isNotEmpty(),
            modifier = Modifier.fillMaxWidth()
        ) {
            Icon(Icons.Default.Send, contentDescription = null)
            Spacer(modifier = Modifier.width(8.dp))
            Text("Send Test Message")
        }

        // Status message
        if (sendStatus.isNotEmpty()) {
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = sendStatus,
                style = MaterialTheme.typography.bodySmall,
                color = if (sendStatus.startsWith("✅"))
                    MaterialTheme.colorScheme.primary
                else
                    MaterialTheme.colorScheme.error
            )
        }

        Spacer(modifier = Modifier.height(12.dp))

        // Quick actions
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            OutlinedButton(
                onClick = {
                    service?.startBle()
                    sendStatus = "BLE restarted"
                },
                modifier = Modifier.weight(1f)
            ) {
                Text("Restart BLE", style = MaterialTheme.typography.labelSmall)
            }

            OutlinedButton(
                onClick = {
                    connectionManager.refreshFromService(service)
                    sendStatus = "Devices refreshed"
                },
                modifier = Modifier.weight(1f)
            ) {
                Text("Refresh", style = MaterialTheme.typography.labelSmall)
            }
        }
    }
}
