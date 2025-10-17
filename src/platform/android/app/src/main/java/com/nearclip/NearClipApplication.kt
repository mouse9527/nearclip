package com.nearclip

import android.app.Application
import dagger.hilt.android.HiltAndroidApp

/**
 * NearClip应用入口点
 * 设置Hilt依赖注入
 */
@HiltAndroidApp
class NearClipApplication : Application() {
    override fun onCreate() {
        super.onCreate()
    }
}