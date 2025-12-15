package com.nearclip

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.nearclip.data.NearClipSettings
import com.nearclip.data.SettingsRepository
import com.nearclip.data.SyncRetryStrategy
import com.nearclip.data.settingsDataStore
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

class SettingsViewModel(application: Application) : AndroidViewModel(application) {

    private val repository = SettingsRepository(application.settingsDataStore)

    val settings: StateFlow<NearClipSettings> = repository.settings
        .stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(5000),
            initialValue = NearClipSettings()
        )

    fun setWifiEnabled(enabled: Boolean) {
        viewModelScope.launch {
            repository.setWifiEnabled(enabled)
        }
    }

    fun setBleEnabled(enabled: Boolean) {
        viewModelScope.launch {
            repository.setBleEnabled(enabled)
        }
    }

    fun setAutoConnect(enabled: Boolean) {
        viewModelScope.launch {
            repository.setAutoConnect(enabled)
        }
    }

    fun setSyncNotifications(enabled: Boolean) {
        viewModelScope.launch {
            repository.setSyncNotifications(enabled)
        }
    }

    fun setDefaultRetryStrategy(strategy: SyncRetryStrategy) {
        viewModelScope.launch {
            repository.setDefaultRetryStrategy(strategy)
        }
    }
}
