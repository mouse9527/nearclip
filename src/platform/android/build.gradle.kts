plugins {
    id("com.android.application") version "8.0.0" apply false
    id("org.jetbrains.kotlin.android") version "1.9.20" apply false
    id("com.google.protobuf") version "0.9.4" apply false
}

allprojects {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

task("clean") {
    delete(rootProject.buildDir)
}