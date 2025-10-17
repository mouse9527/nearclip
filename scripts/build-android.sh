#!/bin/bash

# NearClip Android Build Script
# Usage: ./scripts/build-android.sh

set -e

echo "🔨 Building NearClip Android App..."

# Check if Android SDK is available
if [ -z "$ANDROID_HOME" ]; then
    echo "❌ ANDROID_HOME is not set. Please set it to your Android SDK path."
    exit 1
fi

# Navigate to Android project directory
cd "$(dirname "$0")/../src/platform/android"

echo "📦 Cleaning previous builds..."
./gradlew clean

echo "🔧 Building debug APK..."
./gradlew assembleDebug

echo "✅ Android build completed successfully!"
echo "📱 APK location: app/build/outputs/apk/debug/app-debug.apk"