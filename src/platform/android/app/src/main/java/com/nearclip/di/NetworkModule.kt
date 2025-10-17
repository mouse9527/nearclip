package com.nearclip.di

import com.nearclip.data.network.NearClipApi
import com.nearclip.data.network.proto.DeviceProto
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

/**
 * 网络依赖注入模块
 */
@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {

    /**
     * 提供NearClip API实例
     */
    @Provides
    @Singleton
    fun provideNearClipApi(): NearClipApi {
        return NearClipApi()
    }

    /**
     * 提供Protocol Buffers设备管理器
     */
    @Provides
    @Singleton
    fun provideDeviceProtoManager(): DeviceProto.Manager {
        return DeviceProto.Manager()
    }
}