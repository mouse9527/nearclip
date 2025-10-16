package com.nearclip

import android.app.Application
import android.util.Log
import com.nearclip.core.NearClipCore
import com.nearclip.di.appModule
import com.nearclip.service.initializeServices
import org.koin.android.ext.koin.androidContext
import org.koin.core.context.startKoin
import timber.log.Timber

/**
 * NearClip 应用程序类
 *
 * 负责应用程序级别的初始化和依赖注入配置
 */
class NearClipApplication : Application() {

    // 核心实例
    lateinit var nearClipCore: NearClipCore
        private set

    override fun onCreate() {
        super.onCreate()

        // 初始化日志系统
        initializeLogging()

        // 初始化依赖注入
        initializeDependencyInjection()

        // 初始化核心模块
        initializeCore()

        // 初始化后台服务
        initializeServices()

        Timber.i("NearClip application initialized successfully")
    }

    /**
     * 初始化日志系统
     */
    private fun initializeLogging() {
        if (BuildConfig.DEBUG) {
            // 调试模式：详细日志
            Timber.plant(object : Timber.DebugTree() {
                override fun createStackElementTag(element: StackTraceElement): String {
                    // 显示类名和方法名
                    return "${element.className}:${element.methodName}"
                }
            })
        } else {
            // 发布模式：仅记录警告和错误
            Timber.plant(object : Timber.Tree() {
                override fun isLoggable(tag: String?, priority: Int): Boolean {
                    return priority >= Log.WARN
                }

                override fun log(priority: Int, tag: String?, message: String, t: Throwable?) {
                    if (priority >= Log.WARN) {
                        // 在发布环境中，可以发送到崩溃报告服务
                        if (priority == Log.ERROR && t != null) {
                            // TODO: 发送错误报告到服务端
                        }
                    }
                }
            })
        }
    }

    /**
     * 初始化依赖注入
     */
    private fun initializeDependencyInjection() {
        startKoin {
            androidContext(this@NearClipApplication)
            modules(appModule)
        }
        Timber.d("Dependency injection initialized")
    }

    /**
     * 初始化核心模块
     */
    private fun initializeCore() {
        try {
            nearClipCore = NearClipCore.getInstance(this)
            Timber.i("NearClip core initialized successfully")
        } catch (e: Exception) {
            Timber.e(e, "Failed to initialize NearClip core")
            // 在实际应用中，这里可能需要显示错误对话框或重启应用
            throw RuntimeException("Critical initialization failure", e)
        }
    }

    /**
     * 获取核心实例
     */
    fun getCoreInstance(): NearClipCore {
        return if (::nearClipCore.isInitialized) {
            nearClipCore
        } else {
            throw IllegalStateException("NearClip core not initialized")
        }
    }

    /**
     * 应用程序终止时的清理工作
     */
    override fun onTerminate() {
        super.onTerminate()

        try {
            if (::nearClipCore.isInitialized) {
                nearClipCore.shutdown()
                Timber.i("NearClip core shutdown completed")
            }
        } catch (e: Exception) {
            Timber.e(e, "Error during application shutdown")
        }
    }

    /**
     * 内存压力时的处理
     */
    override fun onTrimMemory(level: Int) {
        super.onTrimMemory(level)

        when (level) {
            TRIM_MEMORY_RUNNING_CRITICAL, TRIM_MEMORY_COMPLETE -> {
                // 内存严重不足，清理缓存
                nearClipCore.clearCache()
                Timber.w("Memory critical, cache cleared")
            }
            TRIM_MEMORY_RUNNING_LOW, TRIM_MEMORY_RUNNING_MODERATE -> {
                // 内存较低，减少缓存大小
                nearClipCore.reduceCacheSize()
                Timber.d("Memory low, cache reduced")
            }
        }
    }
}