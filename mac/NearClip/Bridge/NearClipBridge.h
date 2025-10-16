//
//  NearClipBridge.h
//  NearClip
//
//  macOS 与 Rust 核心库的 Objective-C 桥接头文件
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

// 错误码定义
typedef NS_ENUM(NSInteger, NearClipErrorCode) {
    NearClipErrorCodeSuccess = 0,
    NearClipErrorCodeInvalidParameter = -1,
    NearClipErrorCodeBufferTooSmall = -2,
    NearClipErrorCodeCryptoFailed = -3,
    NearClipErrorCodeBLEFailed = -4,
    NearClipErrorCodeTimeout = -5,
    NearClipErrorCodeInternalError = -6
};

// BLE 连接状态
typedef NS_ENUM(NSInteger, NearClipBLEConnectionState) {
    NearClipBLEConnectionStateDisconnected = 0,
    NearClipBLEConnectionStateConnecting = 1,
    NearClipBLEConnectionStateConnected = 2,
    NearClipBLEConnectionStateBonded = 3
};

// 设备信息结构
@interface NearClipBLEDevice : NSObject
@property (nonatomic, strong) NSString *deviceId;
@property (nonatomic, strong) NSString *deviceName;
@property (nonatomic, strong) NSString *deviceAddress;
@property (nonatomic, assign) NSInteger rssi;
@property (nonatomic, strong) NSDictionary<NSString *, NSData *> *serviceData;
@property (nonatomic, strong) NSDate *lastSeen;
@end

// 同步数据类型
typedef NS_ENUM(NSInteger, NearClipSyncDataType) {
    NearClipSyncDataTypeText = 0,
    NearClipSyncDataTypeImage = 1,
    NearClipSyncDataTypeFile = 2,
    NearClipSyncDataTypeURL = 3
};

// 同步数据结构
@interface NearClipSyncData : NSObject
@property (nonatomic, strong) NSString *dataId;
@property (nonatomic, assign) NearClipSyncDataType dataType;
@property (nonatomic, strong) NSData *content;
@property (nonatomic, strong) NSDate *timestamp;
@property (nonatomic, strong) NSString *sourceDevice;
@property (nonatomic, strong) NSString *contentHash;
@end

/**
 * NearClip 核心桥接类
 *
 * 提供与 Rust 核心库的 Objective-C 接口
 */
@interface NearClipBridge : NSObject

/**
 * 获取单例实例
 */
+ (instancetype)sharedInstance;

/**
 * 初始化核心服务
 */
- (NearClipErrorCode)initialize;

/**
 * 清理资源
 */
- (void)cleanup;

/**
 * 启动服务
 */
- (NearClipErrorCode)start;

/**
 * 停止服务
 */
- (NearClipErrorCode)stop;

#pragma mark - 加密服务

/**
 * 生成会话密钥
 */
- (NSData * _Nullable)generateSessionKeyWithError:(NearClipErrorCode *)error;

/**
 * 生成随机 Nonce
 */
- (NSData * _Nullable)generateNonceWithError:(NearClipErrorCode *)error;

/**
 * 加密数据
 */
- (NSData * _Nullable)encryptData:(NSData *)plaintext
                         withKey:(NSData *)key
                        withNonce:(NSData *)nonce
                          error:(NearClipErrorCode *)error;

/**
 * 解密数据
 */
- (NSData * _Nullable)decryptData:(NSData *)ciphertext
                         withKey:(NSData *)key
                        withNonce:(NSData *)nonce
                          error:(NearClipErrorCode *)error;

/**
 * 数字签名
 */
- (NSData * _Nullable)signData:(NSData *)data
                         error:(NearClipErrorCode *)error;

/**
 * 验证签名
 */
- (BOOL)verifyData:(NSData *)data
        withSignature:(NSData *)signature
        withPublicKey:(NSData *)publicKey
              error:(NearClipErrorCode *)error;

/**
 * 生成配对码
 */
- (NSString * _Nullable)generatePairingCodeWithError:(NearClipErrorCode *)error;

/**
 * 获取设备公钥
 */
- (NSData * _Nullable)getDevicePublicKeyWithError:(NearClipErrorCode *)error;

#pragma mark - BLE 服务

/**
 * 开始设备扫描
 */
- (NSArray<NearClipBLEDevice *> * _Nullable)startDeviceScanWithTimeout:(NSTimeInterval)timeout
                                                                  error:(NearClipErrorCode *)error;

/**
 * 停止设备扫描
 */
- (void)stopDeviceScan;

/**
 * 连接到设备
 */
- (NearClipErrorCode)connectToDevice:(NSString *)deviceId;

/**
 * 断开设备连接
 */
- (NearClipErrorCode)disconnectFromDevice:(NSString *)deviceId;

/**
 * 发送消息到设备
 */
- (NearClipErrorCode)sendMessage:(NSData *)message
                      toDevice:(NSString *)deviceId;

/**
 * 开始广播
 */
- (NearClipErrorCode)startAdvertisingWithDeviceInfo:(NSData *)deviceInfo;

/**
 * 停止广播
 */
- (void)stopAdvertising;

/**
 * 获取设备连接状态
 */
- (NearClipBLEConnectionState)getConnectionStateForDevice:(NSString *)deviceId;

/**
 * 获取已连接的设备列表
 */
- (NSArray<NearClipBLEDevice *> * _Nullable)getConnectedDevicesWithError:(NearClipErrorCode *)error;

#pragma mark - 同步服务

/**
 * 同步数据到所有设备
 */
- (NearClipErrorCode)syncDataToAllDevices:(NearClipSyncData *)data;

/**
 * 处理接收到的同步数据
 */
- (NearClipSyncData * _Nullable)handleReceivedData:(NSData *)data
                                             error:(NearClipErrorCode *)error;

/**
 * 获取同步历史
 */
- (NSArray<NearClipSyncData *> * _Nullable)getSyncHistoryWithError:(NearClipErrorCode *)error;

/**
 * 清除同步历史
 */
- (NearClipErrorCode)clearSyncHistory;

#pragma mark - 工具方法

/**
 * 获取错误描述
 */
- (NSString *)getErrorMessage:(NearClipErrorCode)errorCode;

/**
 * 检查 BLE 是否可用
 */
- (BOOL)isBLEAvailable;

/**
 * 检查权限状态
 */
- (BOOL)hasRequiredPermissions;

/**
 * 请求必需权限
 */
- (void)requestRequiredPermissionsWithCompletion:(void(^)(BOOL granted))completion;

@end

NS_ASSUME_NONNULL_END