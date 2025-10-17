#include "jni_bridge.h"
#include <android/log.h>

#define TAG "NearClipJNICallbacks"
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, TAG, __VA_ARGS__)

// 外部全局变量声明
extern JavaVM* g_jvm;
extern jmethodID g_onDeviceDiscovered;
extern jmethodID g_onDiscoveryStateChanged;
extern jmethodID g_onConnectionChanged;
extern jmethodID g_onClipboardDataReceived;
extern jmethodID g_onError;

/**
 * 获取JNI环境
 */
JNIEnv* get_jni_env() {
    JNIEnv* env = nullptr;
    if (g_jvm == nullptr) {
        LOGE("JavaVM is null");
        return nullptr;
    }

    jint result = g_jvm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6);
    if (result == JNI_EDETACHED) {
        // 当前线程未附加到JVM，尝试附加
        if (!attach_current_thread(&env)) {
            LOGE("Failed to attach current thread to JVM");
            return nullptr;
        }
    } else if (result != JNI_OK) {
        LOGE("Failed to get JNI environment: %d", result);
        return nullptr;
    }

    return env;
}

/**
 * 附加当前线程到JVM
 */
bool attach_current_thread(JNIEnv** env) {
    if (g_jvm == nullptr) {
        return false;
    }

    JavaVMAttachArgs args = {
        JNI_VERSION_1_6,
        "NearClipNativeThread",
        nullptr
    };

    jint result = g_jvm->AttachCurrentThread(env, &args);
    if (result != JNI_OK) {
        LOGE("Failed to attach current thread: %d", result);
        return false;
    }

    return true;
}

/**
 * 分离当前线程
 */
void detach_current_thread() {
    if (g_jvm != nullptr) {
        g_jvm->DetachCurrentThread();
    }
}

/**
 * 调用Java方法：onDeviceDiscovered
 */
void call_java_method_onDeviceDiscovered(const std::string& device_json) {
    JNIEnv* env = get_jni_env();
    if (!env) {
        LOGE("Failed to get JNI environment for onDeviceDiscovered");
        return;
    }

    bool should_detach = false;
    if (env->ExceptionCheck()) {
        env->ExceptionDescribe();
        env->ExceptionClear();
    }

    try {
        // 查找NearClipFFI类
        jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI");
        if (!clazz) {
            LOGE("Failed to find NearClipFFI class");
            if (should_detach) detach_current_thread();
            return;
        }

        // 创建Java字符串
        jstring device_json_str = env->NewStringUTF(device_json.c_str());
        if (!device_json_str) {
            LOGE("Failed to create Java string for device_json");
            if (should_detach) detach_current_thread();
            return;
        }

        // 调用静态方法
        env->CallStaticVoidMethod(clazz, g_onDeviceDiscovered, device_json_str);

        // 清理本地引用
        env->DeleteLocalRef(device_json_str);
        env->DeleteLocalRef(clazz);

        // 检查是否有异常
        if (env->ExceptionCheck()) {
            env->ExceptionDescribe();
            env->ExceptionClear();
        }
    } catch (const std::exception& e) {
        LOGE("Exception in call_java_method_onDeviceDiscovered: %s", e.what());
    }

    if (should_detach) {
        detach_current_thread();
    }
}

/**
 * 调用Java方法：onDiscoveryStateChanged
 */
void call_java_method_onDiscoveryStateChanged(const std::string& state) {
    JNIEnv* env = get_jni_env();
    if (!env) {
        LOGE("Failed to get JNI environment for onDiscoveryStateChanged");
        return;
    }

    try {
        jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI");
        if (!clazz) return;

        jstring state_str = env->NewStringUTF(state.c_str());
        if (!state_str) {
            env->DeleteLocalRef(clazz);
            return;
        }

        env->CallStaticVoidMethod(clazz, g_onDiscoveryStateChanged, state_str);

        env->DeleteLocalRef(state_str);
        env->DeleteLocalRef(clazz);

        if (env->ExceptionCheck()) {
            env->ExceptionDescribe();
            env->ExceptionClear();
        }
    } catch (const std::exception& e) {
        LOGE("Exception in call_java_method_onDiscoveryStateChanged: %s", e.what());
    }
}

/**
 * 调用Java方法：onConnectionChanged
 */
void call_java_method_onConnectionChanged(const std::string& device_id, bool connected) {
    JNIEnv* env = get_jni_env();
    if (!env) {
        LOGE("Failed to get JNI environment for onConnectionChanged");
        return;
    }

    try {
        jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI");
        if (!clazz) return;

        jstring device_id_str = env->NewStringUTF(device_id.c_str());
        if (!device_id_str) {
            env->DeleteLocalRef(clazz);
            return;
        }

        env->CallStaticVoidMethod(clazz, g_onConnectionChanged, device_id_str, static_cast<jboolean>(connected));

        env->DeleteLocalRef(device_id_str);
        env->DeleteLocalRef(clazz);

        if (env->ExceptionCheck()) {
            env->ExceptionDescribe();
            env->ExceptionClear();
        }
    } catch (const std::exception& e) {
        LOGE("Exception in call_java_method_onConnectionChanged: %s", e.what());
    }
}

/**
 * 调用Java方法：onClipboardDataReceived
 */
void call_java_method_onClipboardDataReceived(const std::string& data) {
    JNIEnv* env = get_jni_env();
    if (!env) {
        LOGE("Failed to get JNI environment for onClipboardDataReceived");
        return;
    }

    try {
        jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI");
        if (!clazz) return;

        jstring data_str = env->NewStringUTF(data.c_str());
        if (!data_str) {
            env->DeleteLocalRef(clazz);
            return;
        }

        env->CallStaticVoidMethod(clazz, g_onClipboardDataReceived, data_str);

        env->DeleteLocalRef(data_str);
        env->DeleteLocalRef(clazz);

        if (env->ExceptionCheck()) {
            env->ExceptionDescribe();
            env->ExceptionClear();
        }
    } catch (const std::exception& e) {
        LOGE("Exception in call_java_method_onClipboardDataReceived: %s", e.what());
    }
}

/**
 * 调用Java方法：onError
 */
void call_java_method_onError(const std::string& error) {
    JNIEnv* env = get_jni_env();
    if (!env) {
        LOGE("Failed to get JNI environment for onError");
        return;
    }

    try {
        jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI");
        if (!clazz) return;

        jstring error_str = env->NewStringUTF(error.c_str());
        if (!error_str) {
            env->DeleteLocalRef(clazz);
            return;
        }

        env->CallStaticVoidMethod(clazz, g_onError, error_str);

        env->DeleteLocalRef(error_str);
        env->DeleteLocalRef(clazz);

        if (env->ExceptionCheck()) {
            env->ExceptionDescribe();
            env->ExceptionClear();
        }
    } catch (const std::exception& e) {
        LOGE("Exception in call_java_method_onError: %s", e.what());
    }
}