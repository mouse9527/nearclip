package com.nearclip.service

import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.view.WindowManager
import android.widget.Toast
import androidx.activity.ComponentActivity

/**
 * A transparent activity that briefly comes to foreground to access clipboard on Android 10+.
 *
 * This activity is invisible and doesn't require any interaction from the user.
 * When it gains focus, it can read the clipboard (since it's now the foreground app),
 * then immediately finishes.
 *
 * This is triggered automatically when clipboard access is denied in background,
 * detected by monitoring system logs (requires READ_LOGS permission granted via ADB):
 *
 *   adb shell pm grant com.nearclip android.permission.READ_LOGS
 *   adb shell appops set com.nearclip SYSTEM_ALERT_WINDOW allow
 */
class ClipboardFloatingActivity : ComponentActivity() {

    companion object {
        private const val TAG = "ClipboardFloating"
        private const val KEY_SHOW_TOAST = "SHOW_TOAST"

        fun getIntent(context: Context, showToast: Boolean = false): Intent {
            return Intent(context.applicationContext, ClipboardFloatingActivity::class.java).apply {
                putExtra(KEY_SHOW_TOAST, showToast)
                flags = Intent.FLAG_ACTIVITY_CLEAR_TASK or Intent.FLAG_ACTIVITY_NEW_TASK
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Make the activity completely transparent and non-intrusive
        window.attributes = window.attributes.apply {
            dimAmount = 0f
            flags = WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS or
                    WindowManager.LayoutParams.FLAG_NOT_TOUCH_MODAL
        }

        // No content view needed - we just need focus
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus) {
            android.util.Log.i(TAG, "Window got focus, reading clipboard")

            // We are now the foreground app, so we can access clipboard
            try {
                val clipboardMonitor = ClipboardMonitor.getInstance()
                if (clipboardMonitor != null) {
                    clipboardMonitor.syncCurrentClipboard()
                    android.util.Log.i(TAG, "Clipboard synced via floating activity")

                    if (shouldShowToast()) {
                        Toast.makeText(this, "Clipboard synced", Toast.LENGTH_SHORT).show()
                    }
                } else {
                    android.util.Log.w(TAG, "ClipboardMonitor not available")
                }
            } catch (e: Exception) {
                android.util.Log.e(TAG, "Failed to sync clipboard: ${e.message}")
            }

            // Finish immediately
            finish()
        }
    }

    private fun shouldShowToast(): Boolean {
        return intent.getBooleanExtra(KEY_SHOW_TOAST, false)
    }
}
