# NearClip ProGuard Rules

# Keep JNA classes for UniFFI
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }

# Keep UniFFI generated classes
-keep class com.nearclip.ffi.** { *; }

# Keep native methods
-keepclasseswithmembernames class * {
    native <methods>;
}
