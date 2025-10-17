#ifndef NEARCLIP_JNI_BRIDGE_H
#define NEARCLIP_JNI_BRIDGE_H

#include <jni.h>
#include <string>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * 设备信息结构体
 */
typedef struct {
    std::string device_id;
    std::string device_name;
    std::string device_type;
    std::string public_key;
} DeviceInfo;

/**
 * JNI回调函数声明
 */
void call_java_method_onDeviceDiscovered(const std::string& device_json);
void call_java_method_onDiscoveryStateChanged(const std::string& state);
void call_java_method_onConnectionChanged(const std::string& device_id, bool connected);
void call_java_method_onClipboardDataReceived(const std::string& data);
void call_java_method_onError(const std::string& error);

/**
 * 线程安全的JNI调用辅助函数
 */
JNIEnv* get_jni_env();
bool attach_current_thread(JNIEnv** env);
void detach_current_thread();

#ifdef __cplusplus
}
#endif

#endif // NEARCLIP_JNI_BRIDGE_H