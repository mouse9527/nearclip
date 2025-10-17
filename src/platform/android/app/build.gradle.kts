plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

// 仓库配置已移至settings.gradle.kts

android {
    namespace = "com.nearclip"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.nearclip"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
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
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.4"
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    // Compose BOM
    implementation(platform("androidx.compose:compose-bom:2023.10.01"))

    // Core Android
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.7.0")
    implementation("androidx.activity:activity-compose:1.8.1")

    // Compose UI
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-extended")

    // Hilt Dependency Injection (暂时注释，后续需要时再启用)
    // implementation("com.google.dagger:hilt-android:2.48")
    // implementation("androidx.hilt:hilt-navigation-compose:1.1.0")
    // kapt("com.google.dagger:hilt-compiler:2.48")

    // Room Database (暂时注释，后续需要时再启用)
    // implementation("androidx.room:room-runtime:2.6.0")
    // implementation("androidx.room:room-ktx:2.6.0")
    // kapt("androidx.room:room-compiler:2.6.0")

    // Navigation
    implementation("androidx.navigation:navigation-compose:2.7.5")

    // ViewModel
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.7.0")

    // Permissions
    implementation("com.google.accompanist:accompanist-permissions:0.32.0")

    // Protocol Buffers (暂时注释，后续需要时再启用)
    // implementation("com.google.protobuf:protobuf-kotlin:3.24.4")

    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")

    // Testing
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.mockito:mockito-core:5.6.0")
    testImplementation("org.mockito.kotlin:mockito-kotlin:5.1.0")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.7.3")
    testImplementation("androidx.arch.core:core-testing:2.2.0")
    testImplementation("org.robolectric:robolectric:4.11.1")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2023.10.01"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    // androidTestImplementation("com.google.dagger:hilt-android-testing:2.48")
    // kaptAndroidTest("com.google.dagger:hilt-compiler:2.48")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}

// Protocol Buffers configuration (暂时注释，后续需要时再启用)
/*
protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.24.4"
    }

    generateProtoTasks {
        all().forEach { task ->
            task.builtins {
                create("java") {
                    option("lite")
                }
                create("kotlin") {
                    option("lite")
                }
            }
        }
    }
}
*/