package com.nearclip.ffi

import android.content.Context
import android.util.Log
import com.nearclip.data.model.Device
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicBoolean
import javax.inject.Inject
import javax.inject.Singleton

/**
 * 安全的FFI包装器
 * 提供输入验证、异常处理和内存安全保护
 */
@Singleton
class SecureFFIWrapper @Inject constructor() {

    companion object {
        private const val TAG = "SecureFFIWrapper"
        private const val MAX_DEVICE_ID_LENGTH = 256
        private const val MAX_CLIPBOARD_DATA_LENGTH = 1024 * 1024 // 1MB
        private const val MAX_CALLBACK_REGISTRATION_COUNT = 100

        // 允许的设备ID字符集
        private val VALID_DEVICE_ID_CHARS = setOf(
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_'
        )
    }

    private val isInitialized = AtomicBoolean(false)
    private val callbackRegistrations = ConcurrentHashMap<String, AtomicBoolean>()
    private val operationCounts = ConcurrentHashMap<String, AtomicLong>()

    // 安全状态流
    private val _securityState = MutableStateFlow<SecurityState>(SecurityState.Safe)
    val securityState: Flow<SecurityState> = _securityState.asStateFlow()

    /**
     * 安全初始化
     */
    fun initializeSecure(context: Context): Result<Boolean> {
        return try {
            // 验证上下文
            if (!validateContext(context)) {
                return Result.failure(SecurityException("Invalid context provided"))
            }

            // 检查是否已初始化
            if (isInitialized.get()) {
                return Result.success(true)
            }

            // 调用底层初始化
            val success = NearClipFFI.initialize(context)
            if (success) {
                isInitialized.set(true)
                _securityState.value = SecurityState.Safe
                Result.success(true)
            } else {
                _securityState.value = SecurityState.Error("Failed to initialize FFI")
                Result.failure(SecurityException("FFI initialization failed"))
            }
        } catch (e: Exception) {
            _securityState.value = SecurityState.Error("Initialization exception: ${e.message}")
            Result.failure(SecurityException("Secure initialization failed", e))
        }
    }

    /**
     * 安全设备发现
     */
    fun startSecureDiscovery(): Result<Boolean> {
        return try {
            validateOperationState("discovery")

            val success = NearClipFFI.startDeviceDiscovery()
            if (success) {
                incrementOperationCount("discovery")
                Result.success(true)
            } else {
                Result.failure(SecurityException("Failed to start device discovery"))
            }
        } catch (e: Exception) {
            _securityState.value = SecurityState.Error("Discovery start failed: ${e.message}")
            Result.failure(SecurityException("Secure discovery failed", e))
        }
    }

    /**
     * 安全设备连接
     */
    fun connectSecureToDevice(deviceId: String): Result<Boolean> {
        return try {
            // 验证设备ID
            if (!validateDeviceId(deviceId)) {
                return Result.failure(SecurityException("Invalid device ID: $deviceId"))
            }

            validateOperationState("connect")

            val success = NearClipFFI.connectToDevice(deviceId)
            if (success) {
                incrementOperationCount("connect")
                Result.success(true)
            } else {
                Result.failure(SecurityException("Failed to connect to device"))
            }
        } catch (e: Exception) {
            _securityState.value = SecurityState.Error("Device connection failed: ${e.message}")
            Result.failure(SecurityException("Secure device connection failed", e))
        }
    }

    /**
     * 安全发送剪贴板数据
     */
    fun sendSecureClipboardData(data: String): Result<Boolean> {
        return try {
            // 验证剪贴板数据
            if (!validateClipboardData(data)) {
                return Result.failure(SecurityException("Invalid clipboard data"))
            }

            validateOperationState("clipboard")

            val success = NearClipFFI.sendClipboardData(data)
            if (success) {
                incrementOperationCount("clipboard")
                Result.success(true)
            } else {
                Result.failure(SecurityException("Failed to send clipboard data"))
            }
        } catch (e: Exception) {
            _securityState.value = SecurityState.Error("Clipboard send failed: ${e.message}")
            Result.failure(SecurityException("Secure clipboard send failed", e))
        }
    }

    /**
     * 安全获取设备信息
     */
    fun getSecureLocalDeviceInfo(): Result<DeviceInfo?> {
        return try {
            validateOperationState("get_info")

            val deviceInfo = NearClipFFI.getLocalDeviceInfo()
            incrementOperationCount("get_info")
            Result.success(deviceInfo)
        } catch (e: Exception) {
            _securityState.value = SecurityState.Error("Get device info failed: ${e.message}")
            Result.failure(SecurityException("Secure get device info failed", e))
        }
    }

    /**
     * 验证上下文
     */
    private fun validateContext(context: Context?): Boolean {
        return context != null && try {
            // 基本验证
            context.applicationInfo != null && context.packageName != null
        } catch (e: Exception) {
            Log.e(TAG, "Context validation failed", e)
            false
        }
    }

    /**
     * 验证设备ID格式
     */
    private fun validateDeviceId(deviceId: String): Boolean {
        // 检查长度
        if (deviceId.isEmpty() || deviceId.length > MAX_DEVICE_ID_LENGTH) {
            Log.w(TAG, "Device ID length invalid: ${deviceId.length}")
            return false
        }

        // 检查字符集
        if (!deviceId.all { it in VALID_DEVICE_ID_CHARS }) {
            Log.w(TAG, "Device ID contains invalid characters: $deviceId")
            return false
        }

        return true
    }

    /**
     * 验证剪贴板数据
     */
    private fun validateClipboardData(data: String): Boolean {
        // 检查大小
        if (data.length > MAX_CLIPBOARD_DATA_LENGTH) {
            Log.w(TAG, "Clipboard data too large: ${data.length} bytes")
            return false
        }

        // 检查是否包含潜在的恶意内容
        if (containsSuspiciousContent(data)) {
            Log.w(TAG, "Clipboard data contains suspicious content")
            return false
        }

        return true
    }

    /**
     * 检查可疑内容
     */
    private fun containsSuspiciousContent(data: String): Boolean {
        val suspiciousPatterns = listOf(
            "<script>", "javascript:", "eval(", "exec(",
            "system(", "runtime.exec", "getClass().forName"
        )

        return suspiciousPatterns.any { pattern ->
            data.lowercase().contains(pattern)
        }
    }

    /**
     * 验证操作状态
     */
    private fun validateOperationState(operation: String) {
        if (!isInitialized.get()) {
            throw SecurityException("FFI not initialized")
        }

        // 检查操作频率
        val count = operationCounts.getOrPut(operation) { AtomicLong(0) }.incrementAndGet()
        if (count > 1000) { // 每个操作最多1000次调用
            throw SecurityException("Operation rate limit exceeded: $operation")
        }
    }

    /**
     * 增加操作计数
     */
    private fun incrementOperationCount(operation: String) {
        operationCounts.getOrPut(operation) { AtomicLong(0) }.incrementAndGet()
    }

    /**
     * 安全清理
     */
    fun secureCleanup() {
        try {
            if (isInitialized.get()) {
                NearClipFFI.cleanup()
                isInitialized.set(false)
                callbackRegistrations.clear()
                operationCounts.clear()
                _securityState.value = SecurityState.Safe
            }
        } catch (e: Exception) {
            Log.e(TAG, "Cleanup failed", e)
            _securityState.value = SecurityState.Error("Cleanup failed: ${e.message}")
        }
    }

    /**
     * 获取安全统计信息
     */
    fun getSecurityStats(): Map<String, Any> {
        return mapOf(
            "initialized" to isInitialized.get(),
            "callback_registrations" to callbackRegistrations.size,
            "operation_counts" to operationCounts.toMap(),
            "security_state" to _securityState.value
        )
    }
}

/**
 * 安全状态
 */
sealed class SecurityState {
    object Safe : SecurityState()
    object Warning : SecurityState()
    data class Error(val message: String) : SecurityState()
}

/**
 * 安全异常类
 */
class SecurityException(message: String, cause: Throwable? = null) : Exception(message, cause)