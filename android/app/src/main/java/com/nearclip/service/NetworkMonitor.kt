package com.nearclip.service

import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.os.Handler
import android.os.Looper
import android.util.Log

/**
 * Monitors network connectivity and triggers reconnection on recovery.
 */
class NetworkMonitor(private val context: Context) {

    companion object {
        private const val TAG = "NetworkMonitor"
        private const val MAX_RECONNECT_ATTEMPTS = 3
        private const val BASE_RECONNECT_DELAY_MS = 1000L
    }

    private val connectivityManager = context.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
    private val handler = Handler(Looper.getMainLooper())

    private var isRegistered = false
    private var wasDisconnected = false
    private var reconnectAttempts = 0
    private var pendingReconnect: Runnable? = null

    /** Callback when network connectivity is restored */
    var onNetworkRestored: (() -> Unit)? = null

    /** Callback when reconnection fails after max attempts */
    var onReconnectFailed: (() -> Unit)? = null

    /** Callback to check if currently connected to devices */
    var isConnectedToDevices: (() -> Boolean)? = null

    private val networkCallback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) {
            Log.d(TAG, "Network available")
            if (wasDisconnected) {
                wasDisconnected = false
                Log.i(TAG, "Network restored, scheduling reconnection")
                scheduleReconnect()
            }
        }

        override fun onLost(network: Network) {
            Log.d(TAG, "Network lost")
            wasDisconnected = true
        }

        override fun onCapabilitiesChanged(network: Network, networkCapabilities: NetworkCapabilities) {
            val hasInternet = networkCapabilities.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
            val hasValidated = networkCapabilities.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED)
            Log.d(TAG, "Network capabilities changed: hasInternet=$hasInternet, hasValidated=$hasValidated")
        }
    }

    /**
     * Start monitoring network connectivity.
     */
    fun startMonitoring() {
        if (isRegistered) return

        val request = NetworkRequest.Builder()
            .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
            .build()

        try {
            connectivityManager.registerNetworkCallback(request, networkCallback)
            isRegistered = true
            Log.i(TAG, "Started monitoring network connectivity")

            // Check current network state
            val currentNetwork = connectivityManager.activeNetwork
            val capabilities = currentNetwork?.let { connectivityManager.getNetworkCapabilities(it) }
            val hasInternet = capabilities?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true
            wasDisconnected = !hasInternet

        } catch (e: Exception) {
            Log.e(TAG, "Failed to register network callback", e)
        }
    }

    /**
     * Stop monitoring network connectivity.
     */
    fun stopMonitoring() {
        if (!isRegistered) return

        try {
            connectivityManager.unregisterNetworkCallback(networkCallback)
            isRegistered = false
            cancelPendingReconnect()
            Log.i(TAG, "Stopped monitoring network connectivity")
        } catch (e: Exception) {
            Log.e(TAG, "Failed to unregister network callback", e)
        }
    }

    /**
     * Reset reconnection attempts counter.
     */
    fun resetReconnectAttempts() {
        reconnectAttempts = 0
    }

    private fun scheduleReconnect() {
        cancelPendingReconnect()

        val delay = calculateReconnectDelay()
        Log.i(TAG, "Reconnection scheduled in ${delay}ms (attempt ${reconnectAttempts + 1}/$MAX_RECONNECT_ATTEMPTS)")

        val runnable = Runnable {
            attemptReconnect()
        }
        pendingReconnect = runnable
        handler.postDelayed(runnable, delay)
    }

    private fun cancelPendingReconnect() {
        pendingReconnect?.let { handler.removeCallbacks(it) }
        pendingReconnect = null
    }

    private fun calculateReconnectDelay(): Long {
        // Exponential backoff: 1s, 2s, 4s
        return BASE_RECONNECT_DELAY_MS * (1L shl reconnectAttempts)
    }

    private fun attemptReconnect() {
        reconnectAttempts++
        Log.i(TAG, "Attempting reconnection (attempt $reconnectAttempts/$MAX_RECONNECT_ATTEMPTS)")

        // Trigger reconnection
        onNetworkRestored?.invoke()

        // Check result after a delay
        handler.postDelayed({
            val isConnected = isConnectedToDevices?.invoke() ?: false

            if (!isConnected && reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
                // Schedule another attempt
                scheduleReconnect()
            } else if (!isConnected && reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
                // Max attempts reached, notify user
                Log.w(TAG, "Reconnection failed after $MAX_RECONNECT_ATTEMPTS attempts")
                onReconnectFailed?.invoke()
            } else {
                // Successfully reconnected
                Log.i(TAG, "Reconnection successful")
                reconnectAttempts = 0
            }
        }, 5000L)
    }
}
