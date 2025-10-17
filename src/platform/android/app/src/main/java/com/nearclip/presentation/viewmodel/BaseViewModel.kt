package com.nearclip.presentation.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.CoroutineExceptionHandler
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.plus

/**
 * ViewModel基类
 * 提供通用的错误处理和状态管理功能
 */
abstract class BaseViewModel : ViewModel() {

    // 通用的错误处理
    protected val exceptionHandler = CoroutineExceptionHandler { _, throwable ->
        handleError(throwable)
    }

    /**
     * 错误状态
     */
    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error.asStateFlow()

    /**
     * 加载状态
     */
    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()

    /**
     * 启动协程并处理错误
     */
    protected fun launchSafely(block: suspend () -> Unit) {
        viewModelScope.launch(exceptionHandler + Dispatchers.IO) {
            _isLoading.value = true
            try {
                block()
            } finally {
                _isLoading.value = false
            }
        }
    }

    /**
     * 启动UI协程并处理错误
     */
    protected fun launchSafelyOnUI(block: suspend () -> Unit) {
        viewModelScope.launch(exceptionHandler + Dispatchers.Main.immediate) {
            _isLoading.value = true
            try {
                block()
            } finally {
                _isLoading.value = false
            }
        }
    }

    /**
     * 处理错误
     */
    open fun handleError(throwable: Throwable) {
        _error.value = throwable.message ?: "未知错误"
    }

    /**
     * 清除错误状态
     */
    fun clearError() {
        _error.value = null
    }

    /**
     * 设置加载状态
     */
    protected fun setLoading(loading: Boolean) {
        _isLoading.value = loading
    }
}