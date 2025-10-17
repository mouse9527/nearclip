package com.nearclip.di

import android.content.Context
import androidx.room.Room
import com.nearclip.data.database.NearClipDatabase
import com.nearclip.data.database.dao.DeviceDao
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

/**
 * 数据库依赖注入模块
 */
@Module
@InstallIn(SingletonComponent::class)
object DatabaseModule {

    /**
     * 提供Room数据库实例
     */
    @Provides
    @Singleton
    fun provideNearClipDatabase(
        @ApplicationContext context: Context
    ): NearClipDatabase {
        return Room.databaseBuilder(
            context,
            NearClipDatabase::class.java,
            "nearclip_database"
        )
        .fallbackToDestructiveMigration()
        .build()
    }

    /**
     * 提供设备DAO
     */
    @Provides
    fun provideDeviceDao(database: NearClipDatabase): DeviceDao {
        return database.deviceDao()
    }
}