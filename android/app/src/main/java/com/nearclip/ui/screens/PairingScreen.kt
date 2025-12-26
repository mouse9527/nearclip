package com.nearclip.ui.screens

import android.graphics.Bitmap
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.text.selection.SelectionContainer
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Keyboard
import androidx.compose.material.icons.filled.QrCode
import androidx.compose.material.icons.filled.QrCodeScanner
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import kotlinx.coroutines.launch
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.google.zxing.BarcodeFormat
import com.google.zxing.qrcode.QRCodeWriter
import com.nearclip.ConnectionManager
import com.nearclip.LocalNearClipService
import com.nearclip.ui.components.QrScanner

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PairingScreen(
    onNavigateBack: () -> Unit,
    connectionManager: ConnectionManager = viewModel()
) {
    val service = LocalNearClipService.current
    var selectedTab by remember { mutableStateOf(0) }
    var manualCode by remember { mutableStateOf("") }
    var isLoading by remember { mutableStateOf(false) }
    var errorMessage by remember { mutableStateOf<String?>(null) }
    var successMessage by remember { mutableStateOf<String?>(null) }
    val coroutineScope = rememberCoroutineScope()

    val pairingCode = remember {
        connectionManager.generatePairingCode()
    }

    // Helper to add device - uses service if available (preferred) for state consistency
    suspend fun addDevice(code: String): String {
        return if (service != null) {
            android.util.Log.i("PairingScreen", "Using service to add device")
            service.addDeviceFromCode(code)
        } else {
            android.util.Log.i("PairingScreen", "Using connectionManager to add device (service unavailable)")
            connectionManager.addDeviceFromCode(code)
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Add Device") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { paddingValues ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .padding(16.dp)
        ) {
            // Tab Row
            TabRow(selectedTabIndex = selectedTab) {
                Tab(
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 },
                    text = { Text("Show QR") },
                    icon = { Icon(Icons.Default.QrCode, contentDescription = null) }
                )
                Tab(
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 },
                    text = { Text("Scan QR") },
                    icon = { Icon(Icons.Default.QrCodeScanner, contentDescription = null) }
                )
            }

            Spacer(modifier = Modifier.height(24.dp))

            when (selectedTab) {
                0 -> {
                    // Show QR Code
                    ShowQRCodeTab(pairingCode = pairingCode)
                }
                1 -> {
                    // Scan QR Code / Enter Manually
                    ScanQRCodeTab(
                        manualCode = manualCode,
                        onManualCodeChange = { manualCode = it },
                        isLoading = isLoading,
                        errorMessage = errorMessage,
                        onSubmit = {
                            isLoading = true
                            errorMessage = null
                            coroutineScope.launch {
                                try {
                                    val deviceName = addDevice(manualCode)
                                    successMessage = "Paired with $deviceName"
                                    isLoading = false
                                    // addDeviceFromCode now includes auto-connect delay, just show success briefly
                                    kotlinx.coroutines.delay(500)
                                    onNavigateBack()
                                } catch (e: Exception) {
                                    errorMessage = e.message ?: "Failed to add device"
                                    isLoading = false
                                }
                            }
                        },
                        onQrCodeScanned = { code ->
                            isLoading = true
                            errorMessage = null
                            coroutineScope.launch {
                                try {
                                    val deviceName = addDevice(code)
                                    successMessage = "Paired with $deviceName"
                                    isLoading = false
                                    // addDeviceFromCode now includes auto-connect delay, just show success briefly
                                    kotlinx.coroutines.delay(500)
                                    onNavigateBack()
                                } catch (e: Exception) {
                                    errorMessage = e.message ?: "Invalid QR code"
                                    isLoading = false
                                }
                            }
                        },
                        successMessage = successMessage,
                        scannerEnabled = !isLoading
                    )
                }
            }
        }
    }
}

@Composable
fun ShowQRCodeTab(pairingCode: String) {
    val qrBitmap = remember(pairingCode) {
        generateQRCode(pairingCode, 256)
    }

    Column(
        modifier = Modifier.fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Card {
            Column(
                modifier = Modifier.padding(24.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                qrBitmap?.let { bitmap ->
                    Image(
                        bitmap = bitmap.asImageBitmap(),
                        contentDescription = "QR Code",
                        modifier = Modifier.size(200.dp)
                    )
                }

                Spacer(modifier = Modifier.height(16.dp))

                Text(
                    text = "Scan this code with another device",
                    style = MaterialTheme.typography.bodyMedium
                )
            }
        }

        Spacer(modifier = Modifier.height(24.dp))

        Text(
            text = "Or share the pairing code:",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )

        Spacer(modifier = Modifier.height(8.dp))

        SelectionContainer {
            Text(
                text = pairingCode,
                style = MaterialTheme.typography.bodySmall,
                fontFamily = androidx.compose.ui.text.font.FontFamily.Monospace
            )
        }
    }
}

@Composable
fun ScanQRCodeTab(
    manualCode: String,
    onManualCodeChange: (String) -> Unit,
    isLoading: Boolean,
    errorMessage: String?,
    onSubmit: () -> Unit,
    onQrCodeScanned: (String) -> Unit,
    successMessage: String? = null,
    scannerEnabled: Boolean = true
) {
    var showManualInput by remember { mutableStateOf(false) }

    Column(
        modifier = Modifier.fillMaxWidth(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        // Show success message overlay
        if (successMessage != null) {
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 32.dp),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer
                )
            ) {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(24.dp),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Icon(
                        imageVector = Icons.Default.QrCode,
                        contentDescription = null,
                        modifier = Modifier.size(48.dp),
                        tint = MaterialTheme.colorScheme.primary
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = successMessage,
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }
            return
        }

        if (!showManualInput) {
            // QR Scanner
            QrScanner(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(280.dp),
                enabled = scannerEnabled,
                onQrCodeScanned = onQrCodeScanned
            )

            Spacer(modifier = Modifier.height(16.dp))

            // Show loading indicator while pairing
            if (isLoading) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.Center
                ) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(20.dp),
                        strokeWidth = 2.dp
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("Connecting to device...")
                }
                Spacer(modifier = Modifier.height(16.dp))
            }

            // Show error message in scan mode too
            errorMessage?.let { error ->
                Card(
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.errorContainer
                    ),
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Text(
                        text = error,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onErrorContainer,
                        modifier = Modifier.padding(16.dp)
                    )
                }
                Spacer(modifier = Modifier.height(16.dp))
            }

            TextButton(onClick = { showManualInput = true }) {
                Icon(
                    imageVector = Icons.Default.Keyboard,
                    contentDescription = null,
                    modifier = Modifier.size(18.dp)
                )
                Spacer(modifier = Modifier.width(8.dp))
                Text("Enter code manually")
            }
        } else {
            // Manual Input
            Text(
                text = "Enter pairing code:",
                style = MaterialTheme.typography.bodyMedium
            )

            Spacer(modifier = Modifier.height(8.dp))

            OutlinedTextField(
                value = manualCode,
                onValueChange = onManualCodeChange,
                label = { Text("Pairing Code") },
                modifier = Modifier.fillMaxWidth(),
                singleLine = false,
                minLines = 3,
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Ascii),
                isError = errorMessage != null
            )

            errorMessage?.let { error ->
                Text(
                    text = error,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.error,
                    modifier = Modifier.padding(top = 4.dp)
                )
            }

            Spacer(modifier = Modifier.height(16.dp))

            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                OutlinedButton(
                    onClick = { showManualInput = false },
                    modifier = Modifier.weight(1f)
                ) {
                    Icon(
                        imageVector = Icons.Default.QrCodeScanner,
                        contentDescription = null,
                        modifier = Modifier.size(18.dp)
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("Scan QR")
                }

                Button(
                    onClick = onSubmit,
                    enabled = manualCode.isNotBlank() && !isLoading,
                    modifier = Modifier.weight(1f)
                ) {
                    if (isLoading) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(20.dp),
                            strokeWidth = 2.dp
                        )
                    } else {
                        Text("Add Device")
                    }
                }
            }
        }
    }
}

private fun generateQRCode(content: String, size: Int): Bitmap? {
    return try {
        val writer = QRCodeWriter()
        val bitMatrix = writer.encode(content, BarcodeFormat.QR_CODE, size, size)
        val bitmap = Bitmap.createBitmap(size, size, Bitmap.Config.RGB_565)
        for (x in 0 until size) {
            for (y in 0 until size) {
                bitmap.setPixel(x, y, if (bitMatrix[x, y]) 0xFF000000.toInt() else 0xFFFFFFFF.toInt())
            }
        }
        bitmap
    } catch (e: Exception) {
        null
    }
}
