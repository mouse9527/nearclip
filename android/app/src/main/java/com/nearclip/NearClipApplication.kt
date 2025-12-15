package com.nearclip

import android.app.Application
import com.nearclip.ffi.LogLevel
import com.nearclip.ffi.initLogging

class NearClipApplication : Application() {

    override fun onCreate() {
        super.onCreate()

        // Initialize Rust logging
        try {
            initLogging(LogLevel.DEBUG)
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}
