# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.
#
# For more details, see
#   http://developer.android.com/guide/developing/tools/proguard.html

# If your project uses WebView with JS, uncomment the following
# and specify the fully qualified class name to the JavaScript interface
# class:
#-keepclassmembers class fqcn.of.javascript.interface.for.webview {
#   public *;
#}

# Uncomment this to preserve the line number information for
# debugging stack traces.
#-keepattributes SourceFile,LineNumberTable

# If you keep the line number information, uncomment this to
# hide the original source file name.
#-renamesourcefileattributes SourceFile

# Keep FFI native methods
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep Rust FFI classes
-keep class com.nearclip.services.RustNativeBridge { *; }
-keep class com.nearclip.services.NearClipManager { *; }

# Keep Protocol Buffers generated classes
-keep class * extends com.google.protobuf.Message { *; }
-dontwarn com.google.protobuf.**