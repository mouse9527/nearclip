package com.nearclip.ffi

import android.content.Context
import android.util.Log
import java.io.File
import java.io.FileOutputStream

/**
 * JNI库加载器
 * 负责加载和验证Rust编译的本地库
 */
object JniLoader {

    private const val TAG = "JniLoader"
    private const val LIB_NAME = "nearclip_jni"
    private var isLoaded = false

    /**
     * 加载JNI库
     */
    fun loadLibrary(context: Context): Boolean {
        if (isLoaded) {
            return true
        }

        return try {
            // 尝试从标准路径加载
            System.loadLibrary(LIB_NAME)
            isLoaded = true
            Log.i(TAG, "Successfully loaded $LIB_NAME from standard path")
            true
        } catch (e: UnsatisfiedLinkError) {
            Log.w(TAG, "Failed to load $LIB_NAME from standard path: ${e.message}")

            // 尝试从应用私有目录加载
            try {
                loadFromPrivateDir(context)
                isLoaded = true
                Log.i(TAG, "Successfully loaded $LIB_NAME from private directory")
                true
            } catch (ex: Exception) {
                Log.e(TAG, "Failed to load $LIB_NAME: ${ex.message}")
                false
            }
        }
    }

    /**
     * 从应用私有目录加载库
     */
    private fun loadFromPrivateDir(context: Context) {
        val libDir = File(context.applicationInfo.nativeLibraryDir)
        val libFile = File(libDir, "lib$LIB_NAME.so")

        if (libFile.exists()) {
            System.load(libFile.absolutePath)
        } else {
            throw UnsatisfiedLinkError("Native library not found: ${libFile.absolutePath}")
        }
    }

    /**
     * 检查库是否已加载
     */
    fun isLibraryLoaded(): Boolean = isLoaded

    /**
     * 获取库版本信息
     */
    external fun getLibraryVersion(): String

    /**
     * 验证库完整性
     */
    external fun verifyLibrary(): Boolean
}