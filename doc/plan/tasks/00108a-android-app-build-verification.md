# Task 00108a: Android空应用基础构建验证

## 任务描述

按照TDD原则创建一个基础的空Android应用，确保应用可以正常编译、安装和运行在设备上。

## TDD开发要求

### RED阶段 - 编写失败的构建验证测试
```bash
#!/bin/bash

# RED: 测试Android项目构建
test_android_build() {
    # 测试gradle构建
    if [ ! -f "android/gradlew" ]; then
        echo "FAIL: Android gradlew script not found"
        return 1
    fi
    
    # 测试构建是否成功
    cd android
    if ! ./gradlew build; then
        echo "FAIL: Android build failed"
        return 1
    fi
    cd ..
    
    # 测试APK是否生成
    if [ ! -f "android/app/build/outputs/apk/debug/app-debug.apk" ]; then
        echo "FAIL: APK not generated"
        return 1
    fi
    
    echo "PASS: Android build successful"
}
```

### GREEN阶段 - 最小实现
```bash
# 创建基础的Android项目结构
# 1. 创建主工程文件 (settings.gradle)
# 2. 创建应用构建文件 (build.gradle)
# 3. 创建清单文件 (AndroidManifest.xml)
# 4. 创建主Activity (MainActivity.kt)
# 5. 创建资源文件 (strings.xml, styles.xml)
```

### REFACTOR阶段
```bash
# 优化构建配置
# 1. 添加版本管理
# 2. 配置构建变体 (debug/release)
# 3. 优化依赖管理
# 4. 添加自动化测试脚本
```

## 验收标准
- [ ] 创建一个基础的空 Android 应用项目
- [ ] Android 项目可以通过 `./gradlew build` 正常构建
- [ ] 生成的 APK 文件可以安装到模拟器或真机
- [ ] 应用可以在设备上正常启动并显示主界面
- [ ] 应用可以正常关闭而不崩溃
- [ ] Android 项目包含基本的配置文件 (AndroidManifest.xml, build.gradle)
- [ ] 可以使用 `adb install` 安装应用
- [ ] 使用 `adb shell am start` 可以启动应用

## 所属故事
- [Story 001: 设备发现与连接](../stories/001-device-discovery.md)

## 后续任务
- [Task 00108b: Android BLE 设备发现核心](00108b-android-ble-discovery-core.md)
- [Task 00108c: Android WiFi 设备发现核心](00108c-android-wifi-discovery-core.md)
- [Task 00108d: Android 权限管理](00108d-android-permission-management.md)