package com.nearclip.android

import android.app.Application
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class NearClipApplication : Application() {
    
    override fun onCreate() {
        super.onCreate()
    }
}