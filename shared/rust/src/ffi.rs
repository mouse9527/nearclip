//! FFI (Foreign Function Interface) 模块
//!
//! 提供 C API 接口，供 Android JNI 和 Swift FFI 调用

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint8_t};
use std::ptr;
use std::slice;
use crate::crypto::{CryptoService, CryptoError};
use crate::ble::{BLEManager, BLEError};
use crate::error::{NearClipError, Result};

// 导出 C API

/// 创建加密服务实例
#[no_mangle]
pub extern "C" fn nearclip_crypto_create() -> *mut CryptoService {
    match CryptoService::new() {
        Ok(crypto) => Box::into_raw(Box::new(crypto)),
        Err(_) => ptr::null_mut(),
    }
}

/// 销毁加密服务实例
#[no_mangle]
pub extern "C" fn nearclip_crypto_destroy(crypto: *mut CryptoService) {
    if !crypto.is_null() {
        unsafe {
            let _ = Box::from_raw(crypto);
        }
    }
}

/// 生成会话密钥
#[no_mangle]
pub extern "C" fn nearclip_crypto_generate_session_key(
    crypto: *mut CryptoService,
    key_buffer: *mut c_uint8_t,
    key_size: c_int
) -> c_int {
    if crypto.is_null() || key_buffer.is_null() || key_size != 32 {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let crypto = unsafe { &mut *crypto };
    match crypto.generate_session_key() {
        Ok(key) => {
            unsafe {
                ptr::copy_nonoverlapping(key.as_ptr(), key_buffer, 32);
            }
            crate::error::error_codes::SUCCESS
        }
        Err(_) => crate::error::error_codes::CRYPTO_ERROR,
    }
}

/// 加密数据
#[no_mangle]
pub extern "C" fn nearclip_crypto_encrypt(
    crypto: *mut CryptoService,
    plaintext: *const c_uint8_t,
    plaintext_len: c_int,
    key: *const c_uint8_t,
    nonce: *const c_uint8_t,
    ciphertext: *mut c_uint8_t,
    ciphertext_len: *mut c_int
) -> c_int {
    if crypto.is_null() || plaintext.is_null() || key.is_null() || nonce.is_null() || ciphertext.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let crypto = unsafe { &mut *crypto };
    let plaintext_slice = unsafe { slice::from_raw_parts(plaintext, plaintext_len as usize) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let nonce_slice = unsafe { slice::from_raw_parts(nonce, 12) };

    match crypto.encrypt(plaintext_slice, key_slice, nonce_slice) {
        Ok(result) => {
            let buffer_size = unsafe { *ciphertext_len } as usize;
            if result.len() <= buffer_size {
                unsafe {
                    ptr::copy_nonoverlapping(result.as_ptr(), ciphertext, result.len());
                    *ciphertext_len = result.len() as c_int;
                }
                crate::error::error_codes::SUCCESS
            } else {
                crate::error::error_codes::INVALID_PARAMETER // 缓冲区太小
            }
        }
        Err(_) => crate::error::error_codes::CRYPTO_ERROR,
    }
}

/// 解密数据
#[no_mangle]
pub extern "C" fn nearclip_crypto_decrypt(
    crypto: *mut CryptoService,
    ciphertext: *const c_uint8_t,
    ciphertext_len: c_int,
    key: *const c_uint8_t,
    nonce: *const c_uint8_t,
    plaintext: *mut c_uint8_t,
    plaintext_len: *mut c_int
) -> c_int {
    if crypto.is_null() || ciphertext.is_null() || key.is_null() || nonce.is_null() || plaintext.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let crypto = unsafe { &mut *crypto };
    let ciphertext_slice = unsafe { slice::from_raw_parts(ciphertext, ciphertext_len as usize) };
    let key_slice = unsafe { slice::from_raw_parts(key, 32) };
    let nonce_slice = unsafe { slice::from_raw_parts(nonce, 12) };

    match crypto.decrypt(ciphertext_slice, key_slice, nonce_slice) {
        Ok(result) => {
            let buffer_size = unsafe { *plaintext_len } as usize;
            if result.len() <= buffer_size {
                unsafe {
                    ptr::copy_nonoverlapping(result.as_ptr(), plaintext, result.len());
                    *plaintext_len = result.len() as c_int;
                }
                crate::error::error_codes::SUCCESS
            } else {
                crate::error::error_codes::INVALID_PARAMETER // 缓冲区太小
            }
        }
        Err(_) => crate::error::error_codes::CRYPTO_ERROR,
    }
}

/// 生成配对码
#[no_mangle]
pub extern "C" fn nearclip_crypto_generate_pairing_code(
    crypto: *mut CryptoService,
    code_buffer: *mut c_char,
    buffer_size: c_int
) -> c_int {
    if crypto.is_null() || code_buffer.is_null() || buffer_size < 7 { // 6 digits + null terminator
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let crypto = unsafe { &mut *crypto };
    match crypto.generate_pairing_code() {
        Ok(code) => {
            let c_string = CString::new(code).unwrap();
            let code_bytes = c_string.as_bytes_with_nul();
            if code_bytes.len() <= buffer_size as usize {
                unsafe {
                    ptr::copy_nonoverlapping(code_bytes.as_ptr(), code_buffer as *mut u8, code_bytes.len());
                }
                crate::error::error_codes::SUCCESS
            } else {
                crate::error::error_codes::INVALID_PARAMETER
            }
        }
        Err(_) => crate::error::error_codes::CRYPTO_ERROR,
    }
}

/// 创建 BLE 管理器实例
#[no_mangle]
pub extern "C" fn nearclip_ble_create(service_uuid: *const c_char) -> *mut BLEManager {
    if service_uuid.is_null() {
        return ptr::null_mut();
    }

    let uuid_str = unsafe { CStr::from_ptr(service_uuid) }.to_str().unwrap_or("");
    let ble_manager = BLEManager::new(uuid_str.to_string());
    Box::into_raw(Box::new(ble_manager))
}

/// 销毁 BLE 管理器实例
#[no_mangle]
pub extern "C" fn nearclip_ble_destroy(ble: *mut BLEManager) {
    if !ble.is_null() {
        unsafe {
            let _ = Box::from_raw(ble);
        }
    }
}

/// 开始设备扫描
#[no_mangle]
pub extern "C" fn nearclip_ble_start_scan(
    ble: *mut BLEManager,
    timeout_seconds: c_int,
    callback: extern "C" fn(*const c_char, *const c_char, c_int)
) -> c_int {
    if ble.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };

    // 在实际实现中，这里需要使用异步运行时
    // 为了演示，这里只是同步调用回调
    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match ble.start_scan(timeout_seconds as u64).await {
            Ok(devices) => {
                for device in devices {
                    let id_cstring = CString::new(device.id).unwrap();
                    let name_cstring = CString::new(device.name).unwrap();
                    callback(id_cstring.as_ptr(), name_cstring.as_ptr(), device.rssi);
                }
                crate::error::error_codes::SUCCESS
            }
            Err(_) => crate::error::error_codes::BLE_ERROR,
        }
    })
}

/// 停止设备扫描
#[no_mangle]
pub extern "C" fn nearclip_ble_stop_scan(ble: *mut BLEManager) -> c_int {
    if ble.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };
    ble.stop_scan();
    crate::error::error_codes::SUCCESS
}

/// 连接到设备
#[no_mangle]
pub extern "C" fn nearclip_ble_connect(
    ble: *mut BLEManager,
    device_id: *const c_char
) -> c_int {
    if ble.is_null() || device_id.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };
    let device_id_str = unsafe { CStr::from_ptr(device_id) }.to_str().unwrap_or("");

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match ble.connect_to_device(device_id_str).await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::BLE_ERROR,
        }
    })
}

/// 断开设备连接
#[no_mangle]
pub extern "C" fn nearclip_ble_disconnect(
    ble: *mut BLEManager,
    device_id: *const c_char
) -> c_int {
    if ble.is_null() || device_id.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };
    let device_id_str = unsafe { CStr::from_ptr(device_id) }.to_str().unwrap_or("");

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match ble.disconnect_from_device(device_id_str).await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::BLE_ERROR,
        }
    })
}

/// 发送消息
#[no_mangle]
pub extern "C" fn nearclip_ble_send_message(
    ble: *mut BLEManager,
    device_id: *const c_char,
    message: *const c_uint8_t,
    message_len: c_int
) -> c_int {
    if ble.is_null() || device_id.is_null() || message.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };
    let device_id_str = unsafe { CStr::from_ptr(device_id) }.to_str().unwrap_or("");
    let message_slice = unsafe { slice::from_raw_parts(message, message_len as usize) };

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match ble.send_message(device_id_str, message_slice).await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::BLE_ERROR,
        }
    })
}

/// 开始广播
#[no_mangle]
pub extern "C" fn nearclip_ble_start_advertising(
    ble: *mut BLEManager,
    device_info: *const c_uint8_t,
    info_len: c_int
) -> c_int {
    if ble.is_null() || device_info.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };
    let info_slice = unsafe { slice::from_raw_parts(device_info, info_len as usize) };

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match ble.start_advertising(info_slice).await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::BLE_ERROR,
        }
    })
}

/// 停止广播
#[no_mangle]
pub extern "C" fn nearclip_ble_stop_advertising(ble: *mut BLEManager) -> c_int {
    if ble.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let ble = unsafe { &mut *ble };

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match ble.stop_advertising().await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::BLE_ERROR,
        }
    })
}

/// 获取错误描述
#[no_mangle]
pub extern "C" fn nearclip_get_error_message(error_code: c_int) -> *const c_char {
    let message = crate::error::error_message_from_code(error_code);
    CString::new(message).unwrap().into_raw()
}

/// 创建 NearClip 核心实例
#[no_mangle]
pub extern "C" fn nearclip_core_create() -> *mut crate::NearClipCore {
    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return ptr::null_mut();
    }

    runtime.unwrap().block_on(async {
        match crate::NearClipCore::new().await {
            Ok(core) => Box::into_raw(Box::new(core)),
            Err(_) => ptr::null_mut(),
        }
    })
}

/// 销毁 NearClip 核心实例
#[no_mangle]
pub extern "C" fn nearclip_core_destroy(core: *mut crate::NearClipCore) {
    if !core.is_null() {
        unsafe {
            let _ = Box::from_raw(core);
        }
    }
}

/// 启动 NearClip 服务
#[no_mangle]
pub extern "C" fn nearclip_core_start(core: *mut crate::NearClipCore) -> c_int {
    if core.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let core = unsafe { &mut *core };

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match core.start().await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::GENERAL_ERROR,
        }
    })
}

/// 停止 NearClip 服务
#[no_mangle]
pub extern "C" fn nearclip_core_stop(core: *mut crate::NearClipCore) -> c_int {
    if core.is_null() {
        return crate::error::error_codes::INVALID_PARAMETER;
    }

    let core = unsafe { &mut *core };

    let runtime = tokio::runtime::Runtime::new();
    if runtime.is_err() {
        return crate::error::error_codes::INTERNAL_ERROR;
    }

    runtime.unwrap().block_on(async {
        match core.stop().await {
            Ok(_) => crate::error::error_codes::SUCCESS,
            Err(_) => crate::error::error_codes::GENERAL_ERROR,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_ffi() {
        let crypto = nearclip_crypto_create();
        assert!(!crypto.is_null());

        let mut key = [0u8; 32];
        let result = nearclip_crypto_generate_session_key(crypto, key.as_mut_ptr(), 32);
        assert_eq!(result, crate::error::error_codes::SUCCESS);

        // 验证密钥不为空
        assert!(key.iter().any(|&b| b != 0));

        nearclip_crypto_destroy(crypto);
    }

    #[test]
    fn test_ble_ffi() {
        let service_uuid = CString::new("6e400001-b5a3-f393-e0a9-e50e24dcca9e").unwrap();
        let ble = nearclip_ble_create(service_uuid.as_ptr());
        assert!(!ble.is_null());

        nearclip_ble_destroy(ble);
    }

    #[test]
    fn test_error_message() {
        let error_msg_ptr = nearclip_get_error_message(0);
        let error_msg = unsafe { CStr::from_ptr(error_msg_ptr) }.to_str().unwrap();
        assert_eq!(error_msg, "操作成功");
    }
}