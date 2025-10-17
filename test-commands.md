# NearClip Android 测试命令指南

## 单元测试

### 运行所有单元测试
```bash
./gradlew test
```

### 运行特定测试类
```bash
# ViewModel测试
./gradlew test --tests "com.nearclip.presentation.viewmodel.NearClipViewModelTest"

# Repository测试
./gradlew test --tests "com.nearclip.data.repository.DeviceRepositoryTest"

# UI测试
./gradlew test --tests "com.nearclip.ui.screens.HomeScreenTest"
```

### 生成测试覆盖率报告
```bash
./gradlew jacocoTestReport
# 报告位置: app/build/reports/jacoco/jacocoTestReport/html/index.html
```

## 集成测试

### 运行Android Instrumentation测试
```bash
./gradlew connectedAndroidTest
```

### 运行Compose UI测试
```bash
./gradlew connectedAndroidTest --tests "com.nearclip.ui.screens.*"
```

## 持续集成测试

### 运行完整测试套件
```bash
./gradlew test connectedAndroidTest
```

### 检查代码覆盖率
```bash
./gradlew testDebugUnitTestCoverageDebug
```

## 测试最佳实践

1. **快速反馈**: 先运行单元测试，再运行集成测试
2. **覆盖率目标**: 最低60%的代码覆盖率
3. **测试命名**: 使用描述性的测试方法名
4. **Mock策略**: 只mock外部依赖，测试真实业务逻辑
5. **异步测试**: 使用runTest和TestDispatcher处理协程

## 常见问题解决

### 问题1: 测试编译错误
```bash
# 清理并重新编译
./gradlew clean test
```

### 问题2: MockK相关错误
```bash
# 确保MockK版本兼容
./gradlew dependencies --configuration testCompileClasspath
```

### 问题3: 协程测试超时
```bash
# 在测试中设置更长的超时时间
./gradlew test --info
```