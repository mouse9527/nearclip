package com.nearclip.data.database

import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase
import android.content.Context
import com.nearclip.data.database.converters.DeviceTypeConverter
import com.nearclip.data.database.converters.ConnectionStatusConverter
import com.nearclip.data.database.dao.DeviceDao
import javax.inject.Inject
import javax.inject.Singleton

/**
 * NearClip 数据库
 */
@Database(
    entities = [com.nearclip.data.model.Device::class],
    version = 1,
    exportSchema = false
)
@TypeConverters(
    DeviceTypeConverter::class,
    ConnectionStatusConverter::class
)
@Singleton
abstract class NearClipDatabase : RoomDatabase() {

    abstract fun deviceDao(): DeviceDao

    companion object {
        const val DATABASE_NAME = "nearclip_database"

        @Volatile
        private var INSTANCE: NearClipDatabase? = null

        fun getDatabase(context: Context): NearClipDatabase {
            return INSTANCE ?: synchronized(this) {
                val instance = Room.databaseBuilder(
                    context.applicationContext,
                    NearClipDatabase::class.java,
                    DATABASE_NAME
                )
                    .addCallback(NearClipDatabaseCallback())
                    .build()
                INSTANCE = instance
                instance
            }
        }
    }
}

/**
 * 数据库回调
 */
private class NearClipDatabaseCallback : RoomDatabase.Callback() {
    override fun onCreate(db: SupportSQLiteDatabase) {
        super.onCreate(db)
        // 可以在这里插入初始数据
    }
}