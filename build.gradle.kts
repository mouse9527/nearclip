// NearClip 统一构建配置
// 支持 Android + Mac 平台的统一构建系统

plugins {
    id("com.google.protobuf") version "0.9.4" apply false
    id("org.jetbrains.kotlin.jvm") version "1.9.20" apply false
}

// 全局配置
allprojects {
    group = "com.nearclip"
    version = "1.0.0"

    repositories {
        google()
        mavenCentral()
    }

    // 统一编码规范
    tasks.withType<JavaCompile> {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
        options.encoding = "UTF-8"
    }

    tasks.withType<Test> {
        testLogging {
            events("passed", "skipped", "failed")
            exceptionFormat = "full"
        }
    }
}

// 统一版本管理
ext {
    set("kotlinVersion", "1.9.20")
    set("protobufVersion", "3.24.0")
    set("composeVersion", "1.5.4")
    set("hiltVersion", "2.48")
    set("roomVersion", "2.5.0")
}

// Protocol Buffers 生成任务
tasks.register<Exec>("generateProtobuf") {
    group = "build"
    description = "生成 Protocol Buffers 文件"

    // 输入目录
    inputs.dir("src/shared/protocol")

    // 输出目录
    val kotlinOutput = file("src/shared/protocol/generated/kotlin")
    val swiftOutput = file("src/shared/protocol/generated/swift")
    val rustOutput = file("src/shared/protocol/generated/rust")

    outputs.dirs(kotlinOutput, swiftOutput, rustOutput)

    doFirst {
        // 确保输出目录存在
        kotlinOutput.mkdirs()
        swiftOutput.mkdirs()
        rustOutput.mkdirs()
    }

    commandLine("protoc",
        "--proto_path=src/shared/protocol",
        "--kotlin_out=${kotlinOutput.absolutePath}",
        "--swift_out=${swiftOutput.absolutePath}",
        "--rust_out=${rustOutput.absolutePath}",
        fileTree("src/shared/protocol").matching { include("**/*.proto") }.files.map { it.absolutePath }
    )
}

// Rust 构建任务
tasks.register<Exec>("buildRust") {
    group = "build"
    description = "构建 Rust 共享库"

    dependsOn("generateProtobuf")
    workingDir(file("src/shared/rust"))

    // 设置环境变量
    environment("PROTO_ROOT", file("src/shared/protocol/generated").absolutePath)

    commandLine("cargo", "build", "--release")

    outputs.dir("src/shared/rust/target/release")
}

// Android 构建任务
tasks.register<Exec>("buildAndroid") {
    group = "build"
    description = "构建 Android 应用"

    dependsOn("generateProtobuf", "buildRust")
    workingDir(file("src/platform/android"))

    doFirst {
        // 复制 Rust 库到 Android 项目
        copy {
            from("src/shared/rust/target/release")
            into("src/platform/android/app/src/main/jniLibs")
            include("libnearclip_*.so", "libnearclip_*.dylib")
        }

        // 复制 Protocol Buffers 生成的文件
        copy {
            from("src/shared/protocol/generated/kotlin")
            into("src/platform/android/app/src/main/java")
        }
    }

    commandLine("./gradlew", "assembleRelease")

    outputs.dir("src/platform/android/app/build/outputs/apk/release")
}

// macOS 构建任务
tasks.register<Exec>("buildMac") {
    group = "build"
    description = "构建 macOS 应用"

    dependsOn("generateProtobuf", "buildRust")
    workingDir(file("src/platform/mac"))

    // 设置环境变量
    environment("RUST_LIB_PATH", file("src/shared/rust/target/release").absolutePath)
    environment("PROTO_ROOT", file("src/shared/protocol/generated").absolutePath)

    doFirst {
        // 复制 Protocol Buffers 生成的文件
        copy {
            from("src/shared/protocol/generated/swift")
            into("src/platform/mac/Sources/NearClip/Generated")
        }
    }

    commandLine("swift", "build", "-c", "release")

    outputs.dir("src/platform/mac/.build/release")
}

// Rust 测试任务
tasks.register<Exec>("testRust") {
    group = "verification"
    description = "运行 Rust 测试"

    workingDir(file("src/shared/rust"))
    commandLine("cargo", "test")
}

// Android 测试任务
tasks.register<Exec>("testAndroid") {
    group = "verification"
    description = "运行 Android 测试"

    workingDir(file("src/platform/android"))
    commandLine("./gradlew", "test")
}

// 统一构建任务 (Android + Mac)
tasks.register("buildAll") {
    group = "build"
    description = "构建所有平台 (Android + Mac)"

    dependsOn("buildRust", "buildAndroid", "buildMac")

    doLast {
        println("✅ 所有平台构建完成!")
        println("📦 Android APK: src/platform/android/app/build/outputs/apk/release/")
        println("🍎 macOS App: src/platform/mac/.build/release/")
        println("⚙️ Rust 库: src/shared/rust/target/release/")
        println("📄 Protocol Buffers: src/shared/protocol/generated/")
    }
}

// 统一测试任务
tasks.register("testAll") {
    group = "verification"
    description = "运行所有测试"

    dependsOn("testRust", "testAndroid")

    doLast {
        println("✅ 所有测试完成!")
    }
}

// 清理任务
tasks.register("cleanAll") {
    group = "build"
    description = "清理所有构建产物"

    doLast {
        // 清理 Rust
        exec {
            workingDir(file("src/shared/rust"))
            commandLine("cargo", "clean")
        }

        // 清理 Android
        exec {
            workingDir(file("src/platform/android"))
            commandLine("./gradlew", "clean")
        }

        // 清理 macOS
        exec {
            workingDir(file("src/platform/mac"))
            commandLine("swift", "package", "clean")
        }

        // 清理生成的文件
        delete("src/shared/protocol/generated")

        println("✅ 清理完成!")
    }
}

// 开发模式构建 (快速构建，跳过 Rust 优化)
tasks.register("buildDev") {
    group = "build"
    description = "开发模式构建 (快速构建)"

    dependsOn("generateProtobuf")

    doLast {
        // 构建 Rust (debug 模式)
        exec {
            workingDir(file("src/shared/rust"))
            commandLine("cargo", "build")
        }

        // 构建 Android (debug 模式)
        exec {
            workingDir(file("src/platform/android"))
            commandLine("./gradlew", "assembleDebug")
        }

        // 构建 macOS (debug 模式)
        exec {
            workingDir(file("src/platform/mac"))
            commandLine("swift", "build", "-c", "debug")
        }

        println("✅ 开发模式构建完成!")
    }
}