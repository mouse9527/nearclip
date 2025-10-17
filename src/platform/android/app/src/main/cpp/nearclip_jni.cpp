#include <jni.h>
#include <android/log.h>
#include <string>
#include <memory>
#include "nearclip_jni.h"
#include "jni_bridge.h"

#define TAG "NearClipJNI"
#define LOGD(...) __android_log_print(ANDROID_LOG_DEBUG, TAG, __VA_ARGS__)
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO, TAG, __VA_ARGS__)
#define LOGW(...) __android_log_print(ANDROID_LOG_WARN, TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, TAG, __VA_ARGS__)

// 全局JNI引用
static JavaVM* g_jvm = nullptr;
static jobject g_context = nullptr;
static jmethodID g_onDeviceDiscovered = nullptr;
static jmethodID g_onDiscoveryStateChanged = nullptr;
static jmethodID g_onConnectionChanged = nullptr;
static jmethodID g_onClipboardDataReceived = nullptr;
static jmethodID g_onError = nullptr;

// Rust核心实例
static std::unique_ptr<NearClipCore> g_nearclip_core = nullptr;

/**
 * JNI_OnLoad - 在库加载时调用
 */
JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void* reserved) {
    LOGI("JNI_OnLoad called");

    g_jvm = vm;

    JNIEnv* env;
    if (vm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6) != JNI_OK) {
        LOGE("Failed to get JNI environment");
        return JNI_ERR;
    }

    // 查找Java类和方法
    jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI");
    if (!clazz) {
        LOGE("Failed to find NearClipFFI class");
        return JNI_ERR;
    }

    // 获取方法ID
    g_onDeviceDiscovered = env->GetStaticMethodID(clazz, "onDeviceDiscovered", "(Ljava/lang/String;)V");
    g_onDiscoveryStateChanged = env->GetStaticMethodID(clazz, "onDiscoveryStateChanged", "(Ljava/lang/String;)V");
    g_onConnectionChanged = env->GetStaticMethodID(clazz, "onConnectionChanged", "(Ljava/lang/String;Z)V");
    g_onClipboardDataReceived = env->GetStaticMethodID(clazz, "onClipboardDataReceived", "(Ljava/lang/String;)V");
    g_onError = env->GetStaticMethodID(clazz, "onError", "(Ljava/lang/String;)V");

    if (!g_onDeviceDiscovered || !g_onDiscoveryStateChanged || !g_onConnectionChanged ||
        !g_onClipboardDataReceived || !g_onError) {
        LOGE("Failed to get method IDs");
        return JNI_ERR;
    }

    LOGI("JNI_OnLoad completed successfully");
    return JNI_VERSION_1_6;
}

/**
 * JNI_OnUnload - 在库卸载时调用
 */
JNIEXPORT void JNICALL JNI_OnUnload(JavaVM* vm, void* reserved) {
    LOGI("JNI_OnUnload called");

    // 清理Rust核心
    if (g_nearclip_core) {
        g_nearclip_core.reset();
    }

    // 清理全局引用
    if (g_context) {
        JNIEnv* env;
        if (vm->GetEnv(reinterpret_cast<void**>(&env), JNI_VERSION_1_6) == JNI_OK) {
            env->DeleteGlobalRef(g_context);
        }
    }

    g_jvm = nullptr;
}

/**
 * 初始化NearClip核心
 */
JNIEXPORT jboolean JNICALL
Java_com_nearclip_ffi_NearClipFFI_initialize(JNIEnv* env, jobject thiz, jobject context) {
    LOGI("Initializing NearClip core");

    try {
        // 保存全局Context引用
        if (g_context) {
            env->DeleteGlobalRef(g_context);
        }
        g_context = env->NewGlobalRef(context);

        // 创建Rust核心实例
        g_nearclip_core = std::make_unique<NearClipCore>();

        // 初始化Rust核心
        bool result = g_nearclip_core->initialize();

        if (result) {
            // 设置回调函数
            setup_callbacks();
            LOGI("NearClip core initialized successfully");
        } else {
            LOGE("Failed to initialize NearClip core");
        }

        return static_cast<jboolean>(result);
    } catch (const std::exception& e) {
        LOGE("Exception in initialize: %s", e.what());
        return JNI_FALSE;
    }
}

/**
 * 开始设备发现
 */
JNIEXPORT jboolean JNICALL
Java_com_nearclip_ffi_NearClipFFI_startDeviceDiscovery(JNIEnv* env, jobject thiz) {
    LOGI("Starting device discovery");

    if (!g_nearclip_core) {
        LOGE("NearClip core not initialized");
        return JNI_FALSE;
    }

    try {
        bool result = g_nearclip_core->start_discovery();
        LOGI("Device discovery started: %s", result ? "success" : "failed");
        return static_cast<jboolean>(result);
    } catch (const std::exception& e) {
        LOGE("Exception in startDeviceDiscovery: %s", e.what());
        return JNI_FALSE;
    }
}

/**
 * 停止设备发现
 */
JNIEXPORT jboolean JNICALL
Java_com_nearclip_ffi_NearClipFFI_stopDeviceDiscovery(JNIEnv* env, jobject thiz) {
    LOGI("Stopping device discovery");

    if (!g_nearclip_core) {
        LOGE("NearClip core not initialized");
        return JNI_FALSE;
    }

    try {
        bool result = g_nearclip_core->stop_discovery();
        LOGI("Device discovery stopped: %s", result ? "success" : "failed");
        return static_cast<jboolean>(result);
    } catch (const std::exception& e) {
        LOGE("Exception in stopDeviceDiscovery: %s", e.what());
        return JNI_FALSE;
    }
}

/**
 * 连接到设备
 */
JNIEXPORT jboolean JNICALL
Java_com_nearclip_ffi_NearClipFFI_connectToDevice(JNIEnv* env, jobject thiz, jstring device_id) {
    const char* device_id_str = env->GetStringUTFChars(device_id, nullptr);
    LOGI("Connecting to device: %s", device_id_str);

    if (!g_nearclip_core) {
        LOGE("NearClip core not initialized");
        env->ReleaseStringUTFChars(device_id, device_id_str);
        return JNI_FALSE;
    }

    try {
        std::string device_id_cpp(device_id_str);
        bool result = g_nearclip_core->connect_to_device(device_id_cpp);
        LOGI("Device connection: %s", result ? "success" : "failed");

        env->ReleaseStringUTFChars(device_id, device_id_str);
        return static_cast<jboolean>(result);
    } catch (const std::exception& e) {
        LOGE("Exception in connectToDevice: %s", e.what());
        env->ReleaseStringUTFChars(device_id, device_id_str);
        return JNI_FALSE;
    }
}

/**
 * 断开设备连接
 */
JNIEXPORT jboolean JNICALL
Java_com_nearclip_ffi_NearClipFFI_disconnectFromDevice(JNIEnv* env, jobject thiz, jstring device_id) {
    const char* device_id_str = env->GetStringUTFChars(device_id, nullptr);
    LOGI("Disconnecting from device: %s", device_id_str);

    if (!g_nearclip_core) {
        LOGE("NearClip core not initialized");
        env->ReleaseStringUTFChars(device_id, device_id_str);
        return JNI_FALSE;
    }

    try {
        std::string device_id_cpp(device_id_str);
        bool result = g_nearclip_core->disconnect_from_device(device_id_cpp);
        LOGI("Device disconnection: %s", result ? "success" : "failed");

        env->ReleaseStringUTFChars(device_id, device_id_str);
        return static_cast<jboolean>(result);
    } catch (const std::exception& e) {
        LOGE("Exception in disconnectFromDevice: %s", e.what());
        env->ReleaseStringUTFChars(device_id, device_id_str);
        return JNI_FALSE;
    }
}

/**
 * 发送剪贴板数据
 */
JNIEXPORT jboolean JNICALL
Java_com_nearclip_ffi_NearClipFFI_sendClipboardData(JNIEnv* env, jobject thiz, jstring data) {
    const char* data_str = env->GetStringUTFChars(data, nullptr);
    LOGD("Sending clipboard data: %s", data_str);

    if (!g_nearclip_core) {
        LOGE("NearClip core not initialized");
        env->ReleaseStringUTFChars(data, data_str);
        return JNI_FALSE;
    }

    try {
        std::string data_cpp(data_str);
        bool result = g_nearclip_core->send_clipboard_data(data_cpp);

        env->ReleaseStringUTFChars(data, data_str);
        return static_cast<jboolean>(result);
    } catch (const std::exception& e) {
        LOGE("Exception in sendClipboardData: %s", e.what());
        env->ReleaseStringUTFChars(data, data_str);
        return JNI_FALSE;
    }
}

/**
 * 获取本地设备信息
 */
JNIEXPORT jobject JNICALL
Java_com_nearclip_ffi_NearClipFFI_getLocalDeviceInfo(JNIEnv* env, jobject thiz) {
    LOGI("Getting local device info");

    if (!g_nearclip_core) {
        LOGE("NearClip core not initialized");
        return nullptr;
    }

    try {
        DeviceInfo info = g_nearclip_core->get_local_device_info();

        // 创建Java DeviceInfo对象
        jclass clazz = env->FindClass("com/nearclip/ffi/NearClipFFI$DeviceInfo");
        jmethodID constructor = env->GetMethodID(clazz, "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V");

        jstring device_id = env->NewStringUTF(info.device_id.c_str());
        jstring device_name = env->NewStringUTF(info.device_name.c_str());
        jstring device_type = env->NewStringUTF(info.device_type.c_str());
        jstring public_key = env->NewStringUTF(info.public_key.c_str());

        jobject result = env->NewObject(clazz, constructor, device_id, device_name, device_type, public_key);

        // 清理本地引用
        env->DeleteLocalRef(device_id);
        env->DeleteLocalRef(device_name);
        env->DeleteLocalRef(device_type);
        env->DeleteLocalRef(public_key);

        return result;
    } catch (const std::exception& e) {
        LOGE("Exception in getLocalDeviceInfo: %s", e.what());
        return nullptr;
    }
}

/**
 * 清理资源
 */
JNIEXPORT void JNICALL
Java_com_nearclip_ffi_NearClipFFI_cleanup(JNIEnv* env, jobject thiz) {
    LOGI("Cleaning up NearClip core");

    if (g_nearclip_core) {
        g_nearclip_core.reset();
    }

    if (g_context) {
        env->DeleteGlobalRef(g_context);
        g_context = nullptr;
    }
}

/**
 * 设置回调函数
 */
void setup_callbacks() {
    if (!g_nearclip_core) return;

    // 设置设备发现回调
    g_nearclip_core->set_device_discovered_callback([](const std::string& device_json) {
        call_java_method_onDeviceDiscovered(device_json);
    });

    // 设置发现状态回调
    g_nearclip_core->set_discovery_state_callback([](const std::string& state) {
        call_java_method_onDiscoveryStateChanged(state);
    });

    // 设置连接状态回调
    g_nearclip_core->set_connection_callback([](const std::string& device_id, bool connected) {
        call_java_method_onConnectionChanged(device_id, connected);
    });

    // 设置剪贴板数据回调
    g_nearclip_core->set_clipboard_callback([](const std::string& data) {
        call_java_method_onClipboardDataReceived(data);
    });

    // 设置错误回调
    g_nearclip_core->set_error_callback([](const std::string& error) {
        call_java_method_onError(error);
    });
}