package com.nearclip

import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.Bundle
import android.os.IBinder
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.compositionLocalOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.Modifier
import androidx.navigation.compose.rememberNavController
import com.nearclip.service.ACTION_SYNC_CLIPBOARD
import com.nearclip.service.NearClipAccessibilityService
import com.nearclip.service.NearClipService
import com.nearclip.ui.navigation.NearClipNavHost
import com.nearclip.ui.theme.NearClipTheme

// Composition local to provide service access throughout the app
val LocalNearClipService = compositionLocalOf<NearClipService?> { null }

class MainActivity : ComponentActivity() {

    private var nearClipService: NearClipService? = null
    private val serviceState = mutableStateOf<NearClipService?>(null)
    private var isBound = false

    private val serviceConnection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            val binder = service as NearClipService.LocalBinder
            nearClipService = binder.getService()
            serviceState.value = nearClipService
            isBound = true
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            nearClipService = null
            serviceState.value = null
            isBound = false
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Check if launched from clipboard sync notification
        handleSyncIntent(intent)

        setContent {
            NearClipTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    CompositionLocalProvider(LocalNearClipService provides serviceState.value) {
                        val navController = rememberNavController()
                        NearClipNavHost(navController = navController)
                    }
                }
            }
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        handleSyncIntent(intent)
    }

    private fun handleSyncIntent(intent: Intent?) {
        if (intent?.action == ACTION_SYNC_CLIPBOARD) {
            android.util.Log.i("MainActivity", "Sync clipboard intent received")
            // Clear pending sync flag
            NearClipAccessibilityService.clearPendingSync()

            // Trigger clipboard sync after a short delay to ensure service is bound
            window.decorView.postDelayed({
                syncClipboardNow()
            }, 500)
        }
    }

    private fun syncClipboardNow() {
        val service = nearClipService
        if (service != null) {
            android.util.Log.i("MainActivity", "Triggering clipboard sync")
            service.syncClipboardNow()
            Toast.makeText(this, "Clipboard synced", Toast.LENGTH_SHORT).show()
        } else {
            android.util.Log.w("MainActivity", "Service not available for clipboard sync")
            Toast.makeText(this, "Service not ready, please try again", Toast.LENGTH_SHORT).show()
        }
    }

    override fun onStart() {
        super.onStart()
        // Bind to service if it's running
        Intent(this, NearClipService::class.java).also { intent ->
            bindService(intent, serviceConnection, Context.BIND_AUTO_CREATE)
        }
    }

    override fun onStop() {
        super.onStop()
        if (isBound) {
            unbindService(serviceConnection)
            isBound = false
        }
    }
}
