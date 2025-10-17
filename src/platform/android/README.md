# NearClip Android 构建指南

## 环境要求

- **JDK**: 17 或更高版本
- **Android SDK**: API 34 (Android 14)
- **Gradle**: 8.0 (通过wrapper自动管理)

## 首次构建设置

### 1. 设置环境变量

确保已设置以下环境变量：
```bash
export JAVA_HOME=/path/to/your/jdk
export ANDROID_HOME=/path/to/your/android-sdk
export PATH=$PATH:$JAVA_HOME/bin:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools
```

### 2. 接受Android SDK许可

```bash
yes | "$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager" --licenses
```

### 3. 首次运行Gradle

首次运行会自动下载Gradle Wrapper：
```bash
./gradlew --version
```

## 构建命令

### 编译项目
```bash
./gradlew build
```

### 运行测试
```bash
./gradlew test
```

### 生成调试APK
```bash
./gradlew assembleDebug
```

### 安装到设备
```bash
./gradlew installDebug
```

### 清理项目
```bash
./gradlew clean
```

## 项目结构

```
src/platform/android/
├── app/
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/com/nearclip/     # Kotlin源代码
│   │   │   ├── res/                   # Android资源
│   │   │   └── cpp/                   # 原生C++代码
│   │   └── test/                      # 测试代码
│   └── build.gradle.kts               # 应用构建配置
├── build.gradle.kts                    # 项目构建配置
├── settings.gradle.kts                 # 项目设置
├── gradlew                            # Gradle包装器(Unix)
├── gradlew.bat                        # Gradle包装器(Windows)
└── gradle/                            # Gradle包装器配置
```

## 故障排除

### 问题1: "command not found: gradlew"
**解决方案**: 确保在正确的目录中，并且gradlew有执行权限：
```bash
chmod +x gradlew
```

### 问题2: "JAVA_HOME is not set"
**解决方案**: 设置正确的JAVA_HOME环境变量：
```bash
export JAVA_HOME=/path/to/java
```

### 问题3: Android SDK未找到
**解决方案**: 设置ANDROID_HOME环境变量：
```bash
export ANDROID_HOME=/path/to/android-sdk
```

### 问题4: Gradle Wrapper下载失败
**解决方案**: 手动下载gradle-wrapper.jar并放置在gradle/wrapper/目录中

## 开发工作流

1. 修改代码
2. 运行测试: `./gradlew test`
3. 构建应用: `./gradlew assembleDebug`
4. 安装测试: `./gradlew installDebug`
5. 检查日志: `adb logcat -s NearClip`

## 性能优化

### 构建优化
- 使用Gradle守护进程: `./gradlew --daemon`
- 并行构建: `./gradlew --parallel`
- 配置优化: `./gradlew --configure-on-demand`

### APK优化
- 启用代码压缩和混淆
- 使用R8进行资源缩减
- 启用APK签名

## 发布流程

1. 更新版本号 (app/build.gradle.kts)
2. 运行所有测试
3. 生成发布APK: `./gradlew assembleRelease`
4. 签名APK
5. 上传到应用商店