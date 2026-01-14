# Contributing to NearClip

感谢你对 NearClip 的贡献兴趣!本指南将帮助你了解如何参与项目开发。

---

## 📋 目录

- [开发环境设置](#开发环境设置)
- [代码规范](#代码规范)
- [测试要求](#测试要求)
- [CI/CD 流程](#cicd-流程)
- [提交 Pull Request](#提交-pull-request)
- [常见问题](#常见问题)

---

## 🛠️ 开发环境设置

### 1. 前置要求

**Rust 开发**:
- Rust 1.70+ (stable)
- Cargo

**macOS 开发**:
- macOS 12.0+
- Xcode 14+
- Swift 5.9+

**Android 开发**:
- Android Studio
- JDK 17
- Android SDK (API 26+)
- Android NDK r25c

### 2. 克隆项目

```bash
git clone https://github.com/yourusername/nearclip.git
cd nearclip
```

### 3. 构建项目

**Rust 核心**:
```bash
cargo build --workspace
```

**macOS 客户端**:
```bash
cd macos/NearClip
swift build
```

**Android 客户端**:
```bash
cd android
./gradlew assembleDebug
```

---

## 📝 代码规范

### Rust 代码规范

我们遵循标准的 Rust 代码规范:

1. **格式化**: 使用 `rustfmt`
   ```bash
   cargo fmt --all
   ```

2. **Lint**: 使用 `clippy`
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **命名规范**:
   - 模块: `snake_case`
   - 类型: `PascalCase`
   - 函数/变量: `snake_case`
   - 常量: `SCREAMING_SNAKE_CASE`

4. **文档注释**:
   ```rust
   /// Brief description
   ///
   /// # Arguments
   ///
   /// * `param` - Parameter description
   ///
   /// # Returns
   ///
   /// Return value description
   pub fn example(param: &str) -> Result<(), Error> {
       // ...
   }
   ```

### Swift 代码规范

遵循 Swift API Design Guidelines:
- 类型: `PascalCase`
- 函数/变量: `camelCase`
- 使用明确的参数标签

### Kotlin 代码规范

遵循 Android Kotlin Style Guide:
- 类型: `PascalCase`
- 函数/变量: `camelCase`
- 使用 4 空格缩进

---

## 🧪 测试要求

### Rust 测试

#### 1. 单元测试

每个功能模块都应有单元测试:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let result = example_function();
        assert_eq!(result, expected_value);
    }
}
```

#### 2. 集成测试

在 `tests/` 目录下添加集成测试:

```rust
// tests/integration_test.rs
use nearclip_core::*;

#[test]
fn test_integration() {
    // Test cross-module functionality
}
```

#### 3. 运行测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定包的测试
cargo test -p nearclip-core

# 运行特定测试
cargo test test_name

# 显示测试输出
cargo test -- --nocapture
```

#### 4. 测试覆盖率

查看测试覆盖率:

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --workspace --all-features --out Html

# 查看报告
open tarpaulin-report.html
```

### 测试要求标准

| 层级 | 最低覆盖率 | 说明 |
|------|-----------|------|
| 核心模块 | 80%+ | nearclip-core, nearclip-crypto |
| 网络模块 | 75%+ | nearclip-transport, nearclip-ble |
| FFI 层 | 60%+ | nearclip-ffi |
| 总体 | 75%+ | 整个 workspace |

---

## 🔄 CI/CD 流程

### GitHub Actions 工作流

每次 Push 或 Pull Request 都会触发 CI 流程:

#### 1. Rust Job

```yaml
rust:
  - ✅ 格式检查 (cargo fmt)
  - ✅ Lint 检查 (cargo clippy)
  - ✅ 构建 (cargo build)
  - ✅ 测试 (cargo test)
  - ✅ 测试统计
```

**运行时间**: ~5-7 分钟

#### 2. macOS Job

```yaml
macos:
  - ✅ 构建 FFI (cargo build -p nearclip-ffi)
  - ✅ 构建 Swift 包 (swift build)
  - ⚠️ Swift 测试 (swift test, continue-on-error)
```

**运行时间**: ~3-5 分钟

#### 3. Android Job

```yaml
android:
  - ✅ 构建 Rust for Android (cargo ndk)
  - ✅ 构建 APK (./gradlew assembleDebug)
```

**运行时间**: ~4-6 分钟

#### 4. Security Job

```yaml
security:
  - ✅ 依赖审计 (cargo audit)
```

**运行时间**: ~1-2 分钟

#### 5. Documentation Job

```yaml
docs:
  - ✅ 构建文档 (cargo doc)
  - ✅ 上传构建产物
```

**运行时间**: ~2-3 分钟

### CI 状态查看

1. **在 GitHub PR 页面**:
   - 每个 check 会显示状态 (✅ / ❌)
   - 点击 "Details" 查看详细日志

2. **在 Actions 标签页**:
   - 查看所有 workflow 运行历史
   - 下载构建产物

3. **测试统计**:
   - 在每次运行的 Summary 中查看
   - 显示各包的测试数量

### CI 失败处理

#### 格式检查失败

```bash
# 本地修复
cargo fmt --all

# 提交修复
git add .
git commit -m "fix: format code"
```

#### Clippy 警告

```bash
# 查看警告
cargo clippy --all-targets --all-features

# 修复自动修复项
cargo clippy --all-targets --all-features --fix

# 手动修复剩余项
```

#### 测试失败

```bash
# 运行失败的测试
cargo test test_name -- --nocapture

# 调试测试
RUST_LOG=debug cargo test test_name -- --nocapture

# 查看测试输出
cargo test -- --show-output
```

---

## 📤 提交 Pull Request

### 1. 创建分支

```bash
# 创建功能分支
git checkout -b feature/your-feature-name

# 或修复分支
git checkout -b fix/issue-number
```

### 2. 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型 (type)**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式 (不影响功能)
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

**示例**:
```bash
git commit -m "feat(ble): add device discovery timeout"
git commit -m "fix(sync): handle clipboard sync error"
git commit -m "docs(api): update API documentation"
```

### 3. Push 到 GitHub

```bash
git push origin feature/your-feature-name
```

### 4. 创建 Pull Request

1. 在 GitHub 上打开项目
2. 点击 "New Pull Request"
3. 填写 PR 描述:
   - 改动说明
   - 相关 Issue
   - 测试情况
   - 截图 (如果有 UI 改动)

### 5. PR 模板

```markdown
## 改动说明

[描述你的改动]

## 相关 Issue

Closes #[issue_number]

## 测试情况

- [ ] 所有测试通过
- [ ] 添加了新的测试
- [ ] 手动测试通过

## 检查清单

- [ ] 代码已格式化 (cargo fmt)
- [ ] Clippy 检查通过
- [ ] 文档已更新
- [ ] CHANGELOG 已更新 (如果需要)
```

### 6. PR 审查

- CI 必须全部通过 (✅)
- 至少一个维护者 approve
- 解决所有 review comments

### 7. 合并

- 使用 "Squash and merge" 保持历史整洁
- 确保 commit message 符合规范

---

## 🐛 常见问题

### Q1: Cargo build 失败

**问题**: 缺少系统依赖

**解决**:
```bash
# macOS
brew install pkg-config openssl

# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev
```

### Q2: 测试超时

**问题**: 某些测试运行时间过长

**解决**:
```bash
# 增加超时时间
cargo test -- --test-threads=1 --timeout 300
```

### Q3: macOS 构建失败

**问题**: Swift 找不到 Rust 库

**解决**:
```bash
# 确保 Rust FFI 已构建
cargo build -p nearclip-ffi --release

# 确保动态库路径正确
ls -la target/release/libnearclip_ffi.*
```

### Q4: Android 构建失败

**问题**: NDK 路径未设置

**解决**:
```bash
# 设置 NDK 路径
export ANDROID_NDK_HOME=/path/to/ndk

# 或在 local.properties 中设置
echo "ndk.dir=/path/to/ndk" >> android/local.properties
```

### Q5: CI 比本地慢

**原因**: CI 没有缓存或首次运行

**正常情况**:
- 首次运行: 10-15 分钟
- 有缓存: 5-8 分钟

### Q6: 如何查看 CI 日志

1. 进入 GitHub Actions 标签页
2. 点击失败的 workflow
3. 点击失败的 job
4. 展开失败的 step
5. 查看详细日志

### Q7: 测试覆盖率下降

**检查**:
```bash
# 生成覆盖率报告
cargo tarpaulin --workspace --all-features

# 查看哪些文件覆盖率低
cargo tarpaulin --workspace --all-features --out Html
open tarpaulin-report.html
```

**补充测试**:
- 为未覆盖的代码添加测试
- 提高现有测试的覆盖面

---

## 📚 资源

### 文档

- [Rust 文档](https://doc.rust-lang.org/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Swift 文档](https://swift.org/documentation/)
- [Android 开发文档](https://developer.android.com/)

### 项目文档

- [架构设计](docs/architecture.md)
- [API 文档](docs/api/)
- [测试指南](docs/manual-testing-guide.md)

### 社区

- [GitHub Issues](https://github.com/yourusername/nearclip/issues)
- [GitHub Discussions](https://github.com/yourusername/nearclip/discussions)

---

## 📄 许可证

通过贡献代码,你同意你的贡献将按照 MIT 许可证授权。

---

**感谢你的贡献! 🎉**

如有问题,请随时在 Issues 中提问。
