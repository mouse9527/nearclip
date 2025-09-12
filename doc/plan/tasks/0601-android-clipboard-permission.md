# Task 0601: 实现Android剪贴板权限检查 (TDD版本)

## 任务描述

按照TDD原则实现Android平台的剪贴板读取权限检查功能。

## TDD开发要求

### 必须遵循的红绿重构循环

#### 1. RED阶段 - 编写失败的测试
```rust
// android/src/test/java/com/nearclip/clipboard/PermissionTest.kt
import org.junit.Test
import org.junit.Assert.*
import android.content.Context
import android.content.pm.PackageManager

class ClipboardPermissionTest {
    
    @Test
    fun testPermissionGranted() {
        // RED: 测试权限已授予的情况
        val context = mockContextWithPermission(true)
        val checker = ClipboardPermissionChecker(context)
        
        assertTrue(checker.hasClipboardPermission())
    }
    
    @Test
    fun testPermissionDenied() {
        // RED: 测试权限被拒绝的情况
        val context = mockContextWithPermission(false)
        val checker = ClipboardPermissionChecker(context)
        
        assertFalse(checker.hasClipboardPermission())
    }
    
    @Test
    fun testPermissionRequest() {
        // RED: 测试权限请求
        val checker = ClipboardPermissionChecker(mockContext())
        
        assertFalse(checker.shouldRequestPermission())
        checker.requestPermission()
        assertTrue(checker.shouldRequestPermission())
    }
}
```

#### 2. GREEN阶段 - 最小实现让测试通过
```kotlin
// android/src/main/java/com/nearclip/clipboard/ClipboardPermissionChecker.kt
package com.nearclip.clipboard

import android.content.Context
import android.content.pm.PackageManager
import android.Manifest

class ClipboardPermissionChecker(private val context: Context) {
    
    fun hasClipboardPermission(): Boolean {
        return context.checkSelfPermission(Manifest.permission.READ_CLIPBOARD) == 
               PackageManager.PERMISSION_GRANTED
    }
    
    fun shouldRequestPermission(): Boolean {
        return !hasClipboardPermission()
    }
    
    fun requestPermission() {
        // 简化实现，实际需要Activity引用
    }
}
```

#### 3. REFACTOR阶段 - 重构代码
```kotlin
// 重构以消除重复，提高代码质量
package com.nearclip.clipboard

import android.content.Context
import android.content.pm.PackageManager
import android.Manifest
import androidx.activity.result.ActivityResultLauncher

class ClipboardPermissionChecker(
    private val context: Context,
    private val permissionLauncher: ActivityResultLauncher<String>? = null
) {
    
    companion object {
        const val CLIPBOARD_PERMISSION = Manifest.permission.READ_CLIPBOARD
        private const val PERMISSION_REQUEST_CODE = 1001
    }
    
    fun hasClipboardPermission(): Boolean {
        return context.checkSelfPermission(CLIPBOARD_PERMISSION) == 
               PackageManager.PERMISSION_GRANTED
    }
    
    fun shouldRequestPermission(): Boolean {
        return !hasClipboardPermission() && 
               !context.shouldShowRequestPermissionRationale(CLIPBOARD_PERMISSION)
    }
    
    fun shouldShowRationale(): Boolean {
        return !hasClipboardPermission() && 
               context.shouldShowRequestPermissionRationale(CLIPBOARD_PERMISSION)
    }
    
    fun requestPermission() {
        permissionLauncher?.launch(CLIPBOARD_PERMISSION)
    }
    
    fun isPermissionDeniedPermanently(): Boolean {
        return !hasClipboardPermission() && 
               !context.shouldShowRequestPermissionRationale(CLIPBOARD_PERMISSION) &&
               hasRequestedPermissionBefore()
    }
    
    private fun hasRequestedPermissionBefore(): Boolean {
        val prefs = context.getSharedPreferences("nearclip_prefs", Context.MODE_PRIVATE)
        return prefs.getBoolean("clipboard_permission_requested", false)
    }
    
    fun markPermissionRequested() {
        val prefs = context.getSharedPreferences("nearclip_prefs", Context.MODE_PRIVATE)
        prefs.edit().putBoolean("clipboard_permission_requested", true).apply()
    }
}
```

### 测试覆盖率要求
- **单元测试覆盖率**: > 95%

## Clean Architecture要求

作为Android平台的infrastructure层实现：

```kotlin
// android/src/main/java/com/nearclip/infrastructure/ClipboardPermissionChecker.kt
class ClipboardPermissionChecker {
    // Android权限检查实现
}
```

## 任务验收标准

- [ ] 所有测试通过（红绿重构循环完成）
- [ ] 测试覆盖率 > 95%
- [ ] 通过代码审查
- [ ] Android Studio编译无错误
- [ ] 权限检查逻辑正确

## 依赖任务

- [Task 0102: 定义设备状态枚举](0102-device-status-enum.md)

## 后续任务

- [Task 0602: 实现Android剪贴板监听器](0602-android-clipboard-listener.md)
- [Task 0603: 实现Android权限请求对话框](0603-android-permission-dialog.md)