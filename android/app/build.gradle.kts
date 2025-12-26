plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

// Task to build Rust native library and generate Kotlin bindings
tasks.register<Exec>("buildRustLibrary") {
    description = "Build Rust native library for Android"
    workingDir = rootProject.projectDir.parentFile // nearclip root
    commandLine("cargo", "ndk", "-t", "arm64-v8a", "-o", "android/app/src/main/jniLibs", "build", "--release", "-p", "nearclip-ffi")
}

tasks.register<Exec>("generateKotlinBindings") {
    description = "Generate Kotlin FFI bindings from UDL"
    dependsOn("buildRustLibrary")
    workingDir = rootProject.projectDir.parentFile // nearclip root
    commandLine("cargo", "run", "-p", "nearclip-ffi", "--bin", "uniffi-bindgen", "--",
        "generate", "crates/nearclip-ffi/src/nearclip.udl",
        "--language", "kotlin",
        "--out-dir", "android/app/src/main/java")
}

// Hook into Android build - run before compiling Kotlin
tasks.matching { it.name == "preBuild" }.configureEach {
    dependsOn("generateKotlinBindings")
}

android {
    namespace = "com.nearclip"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.nearclip"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        ndk {
            // Supported ABIs for native libraries
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.6"
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    // JNA for UniFFI bindings
    implementation("net.java.dev.jna:jna:5.14.0@aar")

    // Core Android
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.6.2")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.6.2")
    implementation("androidx.activity:activity-compose:1.8.2")

    // Compose
    implementation(platform("androidx.compose:compose-bom:2023.10.01"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-extended")

    // Navigation
    implementation("androidx.navigation:navigation-compose:2.7.6")

    // QR Code (for pairing)
    implementation("com.google.zxing:core:3.5.2")

    // CameraX (for QR scanning)
    implementation("androidx.camera:camera-camera2:1.3.1")
    implementation("androidx.camera:camera-lifecycle:1.3.1")
    implementation("androidx.camera:camera-view:1.3.1")

    // ML Kit Barcode Scanning
    implementation("com.google.mlkit:barcode-scanning:17.2.0")

    // Accompanist Permissions
    implementation("com.google.accompanist:accompanist-permissions:0.32.0")

    // DataStore for settings persistence
    implementation("androidx.datastore:datastore-preferences:1.0.0")

    // Security Crypto for encrypted storage
    implementation("androidx.security:security-crypto:1.1.0-alpha06")

    // Testing
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2023.10.01"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}
