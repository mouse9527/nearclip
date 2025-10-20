import Foundation
import CoreBluetooth
import Combine

/**
 * 设备扫描器 - 专门的BLE设备扫描服务
 * 负责设备发现、过滤和管理
 */
class DeviceScanner: NSObject, ObservableObject, CBCentralManagerDelegate {

    // MARK: - Properties

    /// 蓝牙管理器
    private var centralManager: CBCentralManager!

    /// 扫描状态
    @Published var isScanning: Bool = false

    /// 蓝牙状态
    @Published var bluetoothState: CBManagerState = .unknown

    /// 发现的设备信息
    @Published var discoveredDevices: [DeviceInfo] = []

    /// NearClip设备列表
    @Published var nearClipDevices: [DeviceInfo] = []

    /// 设备发现回调
    var onDeviceDiscovered: ((DeviceInfo) -> Void)?

    /// 扫描错误回调
    var onScanError: ((Error) -> Void)?

    /// 设备缓存超时时间（秒）
    private let deviceCacheTimeout: TimeInterval = 30.0

    /// 最大缓存设备数量
    private let maxCachedDevices = 50

    /// 设备发现定时器
    private var cleanupTimer: Timer?

    // MARK: - Initialization

    override init() {
        super.init()
        centralManager = CBCentralManager(delegate: self, queue: nil)
        setupCleanupTimer()
    }

    deinit {
        stopScanning()
        cleanupTimer?.invalidate()
    }

    // MARK: - Public Methods

    /// 开始扫描设备
    func startScanning() {
        guard centralManager.state == .poweredOn else {
            let error = DeviceScannerError.bluetoothNotAvailable
            print("❌ 蓝牙不可用: \(error.localizedDescription)")
            onScanError?(error)
            return
        }

        guard !isScanning else {
            print("⚠️ 已在扫描中")
            return
        }

        print("🔍 开始扫描BLE设备...")

        // 配置扫描选项
        let scanOptions: [String: Any] = [
            CBCentralManagerScanOptionAllowDuplicatesKey: false
        ]

        // 开始扫描所有设备
        centralManager.scanForPeripherals(withServices: nil, options: scanOptions)
        isScanning = true
    }

    /// 停止扫描
    func stopScanning() {
        guard isScanning else { return }

        print("🛑 停止扫描")
        centralManager.stopScan()
        isScanning = false
    }

    /// 获取扫描统计信息
    func getScanStats() -> ScanStats {
        let totalDevices = discoveredDevices.count
        let nearClipCount = nearClipDevices.count
        let lastDiscoveryTime = discoveredDevices.max { $0.discoveryTime < $1.discoveryTime }?.discoveryTime ?? Date.distantPast

        return ScanStats(
            totalDevices: totalDevices,
            nearClipDevices: nearClipCount,
            lastDiscoveryTime: lastDiscoveryTime
        )
    }

    /// 清除所有设备
    func clearAllDevices() {
        discoveredDevices.removeAll()
        nearClipDevices.removeAll()
        print("🗑️ 已清除所有设备")
    }

    /// 手动添加测试设备（用于演示）
    func addTestDevice() {
        let testDevice = DeviceInfo(
            id: UUID().uuidString,
            name: "NearClip-Test-\(Int.random(in: 1000...9999))",
            rssi: Int.random(in: -30...-80),
            deviceType: .nearClip,
            discoveryTime: Date(),
            peripheral: nil
        )

        addOrUpdateDevice(testDevice)
    }

    // MARK: - CBCentralManagerDelegate

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        bluetoothState = central.state

        switch central.state {
        case .poweredOn:
            print("✅ 蓝牙已开启，可以开始扫描")
        case .poweredOff:
            print("❌ 蓝牙已关闭")
            stopScanning()
        case .unauthorized:
            print("❌ 蓝牙权限未授权")
            let error = DeviceScannerError.unauthorized
            onScanError?(error)
        case .unsupported:
            print("❌ 设备不支持BLE")
            let error = DeviceScannerError.unsupported
            onScanError?(error)
        case .resetting:
            print("⚠️ 蓝牙正在重置")
        case .unknown:
            print("❓ 蓝牙状态未知")
        @unknown default:
            print("❓ 未知的蓝牙状态")
        }
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String: Any], rssi RSSI: NSNumber) {

        // 解析设备信息
        let deviceInfo = parseDeviceInfo(peripheral: peripheral, advertisementData: advertisementData, rssi: RSSI)

        // 添加或更新设备信息
        addOrUpdateDevice(deviceInfo)
    }

    // MARK: - Private Methods

    /// 解析设备信息
    private func parseDeviceInfo(peripheral: CBPeripheral, advertisementData: [String: Any], rssi: NSNumber) -> DeviceInfo {

        let deviceName = peripheral.name ?? (advertisementData[CBAdvertisementDataLocalNameKey] as? String) ?? "未知设备"
        let deviceId = peripheral.identifier.uuidString

        // 确定设备类型
        let deviceType: DeviceType
        if deviceName.contains("NearClip", options: .caseInsensitive) {
            deviceType = .nearClip
        } else if let manufacturerData = advertisementData[CBAdvertisementDataManufacturerDataKey] as? Data {
            // 根据制造商数据判断设备类型
            deviceType = parseManufacturerData(manufacturerData)
        } else {
            deviceType = .unknown
        }

        return DeviceInfo(
            id: deviceId,
            name: deviceName,
            rssi: rssi.intValue,
            deviceType: deviceType,
            discoveryTime: Date(),
            peripheral: peripheral
        )
    }

    /// 解析制造商数据
    private func parseManufacturerData(_ data: Data) -> DeviceType {
        guard data.count >= 2 else { return .unknown }

        // 提取制造商ID (前2字节)
        let manufacturerId = data.withUnsafeBytes { $0.load(as: UInt16.self) }

        // 根据制造商ID判断设备类型
        switch manufacturerId {
        case 0x004C: // Apple
            return .apple
        case 0x00E0: // Google
            return .google
        default:
            return .other
        }
    }

    /// 添加或更新设备信息
    private func addOrUpdateDevice(_ deviceInfo: DeviceInfo) {
        // 检查是否已存在
        if let existingIndex = discoveredDevices.firstIndex(where: { $0.id == deviceInfo.id }) {
            let existingDevice = discoveredDevices[existingIndex]

            // 判断是否需要更新
            let shouldUpdate = deviceInfo.rssi > existingDevice.rssi + 5 ||
                              deviceInfo.discoveryTime.timeIntervalSince(existingDevice.discoveryTime) > 5.0

            if shouldUpdate {
                discoveredDevices[existingIndex] = deviceInfo
                print("📱 设备信息更新: \(deviceInfo.name) (RSSI: \(deviceInfo.rssi)dBm)")
            }
        } else {
            discoveredDevices.append(deviceInfo)
            print("📱 发现新设备: \(deviceInfo.name) (RSSI: \(deviceInfo.rssi)dBm)")
        }

        // 更新NearClip设备列表
        updateNearClipDevices()

        // 触发回调
        onDeviceDiscovered?(deviceInfo)
    }

    /// 更新NearClip设备列表
    private func updateNearClipDevices() {
        nearClipDevices = discoveredDevices.filter { $0.deviceType == .nearClip }
    }

    /// 设置清理定时器
    private func setupCleanupTimer() {
        cleanupTimer = Timer.scheduledTimer(withTimeInterval: 10.0, repeats: true) { [weak self] _ in
            self?.cleanupOldDevices()
        }
    }

    /// 清理过期设备
    private func cleanupOldDevices() {
        let now = Date()
        let thresholdDate = now.addingTimeInterval(-deviceCacheTimeout)

        // 移除过期设备
        let beforeCount = discoveredDevices.count
        discoveredDevices.removeAll { $0.discoveryTime < thresholdDate }
        let afterCount = discoveredDevices.count

        // 如果设备数量超过限制，移除最旧的设备
        if discoveredDevices.count > maxCachedDevices {
            let sortedDevices = discoveredDevices.sorted { $0.discoveryTime < $1.discoveryTime }
            let excessCount = discoveredDevices.count - maxCachedDevices
            let devicesToRemove = Array(sortedDevices.prefix(excessCount))

            discoveredDevices.removeAll { device in
                devicesToRemove.contains { $0.id == device.id }
            }
        }

        // 更新NearClip设备列表
        updateNearClipDevices()

        if beforeCount > afterCount {
            print("🗑️ 清理过期设备: 移除了 \(beforeCount - afterCount) 个设备")
        }
    }
}

// MARK: - Supporting Types

/// 设备信息
struct DeviceInfo: Identifiable, Equatable {
    let id: String
    let name: String
    let rssi: Int
    let deviceType: DeviceType
    let discoveryTime: Date
    let peripheral: CBPeripheral?

    // Equatable实现（忽略peripheral，因为它可能在不同时间创建）
    static func == (lhs: DeviceInfo, rhs: DeviceInfo) -> Bool {
        return lhs.id == rhs.id &&
               lhs.name == rhs.name &&
               lhs.deviceType == rhs.deviceType
    }
}

/// 设备类型
enum DeviceType: String, CaseIterable {
    case unknown = "未知"
    case nearClip = "NearClip"
    case apple = "Apple"
    case google = "Google"
    case other = "其他"
}

/// 扫描统计信息
struct ScanStats {
    let totalDevices: Int
    let nearClipDevices: Int
    let lastDiscoveryTime: Date

    var lastDiscoveryTimeString: String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter.localizedString(for: lastDiscoveryTime, relativeTo: Date())
    }
}

/// 扫描错误
enum DeviceScannerError: LocalizedError {
    case bluetoothNotAvailable
    case unauthorized
    case unsupported

    var errorDescription: String? {
        switch self {
        case .bluetoothNotAvailable:
            return "蓝牙不可用"
        case .unauthorized:
            return "蓝牙权限未授权"
        case .unsupported:
            return "设备不支持BLE"
        }
    }
}