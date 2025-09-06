//
//  BLEPeripheralService.swift
//  NearClip
//
//  Created by NearClip Team on 2024/01/01.
//

import Foundation
import CoreBluetooth
import os.log
import Combine
import AppKit

class BLEPeripheralService: NSObject, ObservableObject {
    @Published var isAdvertising = false
    @Published var isBluetoothEnabled = false
    @Published var connectedCentrals: [CBCentral] = []
    @Published var lastError: String?
    
    private var peripheralManager: CBPeripheralManager!
    private var nearClipService: CBMutableService!
    private var writeCharacteristic: CBMutableCharacteristic!
    private var readCharacteristic: CBMutableCharacteristic!
    
    // 与 BLEService 相同的 UUID
    private let nearClipServiceUUID = CBUUID(string: "12345678-1234-5678-9012-123456789ABC")
    private let writeCharacteristicUUID = CBUUID(string: "12345678-1234-5678-9012-123456789ABD")
    private let readCharacteristicUUID = CBUUID(string: "12345678-1234-5678-9012-123456789ABE")
    
   override init() {
        super.init()
        print("[BLEPeripheralService] Initializing peripheral service")
        os_log("[BLEPeripheralService] Initializing peripheral service", log: OSLog.default, type: .info)
        setupService()
        peripheralManager = CBPeripheralManager(delegate: self, queue: nil)
        print("[BLEPeripheralService] CBPeripheralManager created with initial state: \(peripheralManager.state.rawValue)")
        os_log("[BLEPeripheralService] CBPeripheralManager created with initial state: %d", log: OSLog.default, type: .info, peripheralManager.state.rawValue)
        print("[BLEPeripheralService] Peripheral service initialization completed")
        os_log("[BLEPeripheralService] Peripheral service initialization completed", log: OSLog.default, type: .info)
    }
    
    private func setupService() {
        // 创建特征
        writeCharacteristic = CBMutableCharacteristic(
            type: writeCharacteristicUUID,
            properties: [.write, .writeWithoutResponse],
            value: nil,
            permissions: [.writeable]
        )
        
        readCharacteristic = CBMutableCharacteristic(
            type: readCharacteristicUUID,
            properties: [.read, .notify],
            value: nil,
            permissions: [.readable]
        )
        
        // 创建服务
        nearClipService = CBMutableService(
            type: nearClipServiceUUID,
            primary: true
        )
        nearClipService.characteristics = [writeCharacteristic, readCharacteristic]
    }
    
    func startAdvertising() {
        print("[BLEPeripheralService] startAdvertising called - current state: \(peripheralManager.state.rawValue)")
        guard peripheralManager.state == .poweredOn else {
            print("[BLEPeripheralService] Cannot start advertising - Bluetooth state: \(peripheralManager.state.rawValue)")
            NSLog("[BLEPeripheralService] Cannot start advertising - Bluetooth state: \(peripheralManager.state.rawValue)")
            lastError = "蓝牙未启用"
            return
        }
        
        guard !isAdvertising else { 
            print("[BLEPeripheralService] Already advertising")
            NSLog("[BLEPeripheralService] Already advertising")
            return 
        }
        
        print("[BLEPeripheralService] Starting advertising with service UUID: \(nearClipServiceUUID.uuidString)")
        NSLog("[BLEPeripheralService] Starting advertising with service UUID: \(nearClipServiceUUID.uuidString)")
        
        // 添加服务
        peripheralManager.add(nearClipService)
        
        // 开始广播
        let deviceName = Host.current().localizedName ?? "Unknown"
        let advertisementData: [String: Any] = [
            CBAdvertisementDataServiceUUIDsKey: [nearClipServiceUUID],
            CBAdvertisementDataLocalNameKey: "NearClip-\(deviceName.prefix(8))"
        ]
        
        print("[BLEPeripheralService] Advertisement data: \(advertisementData)")
        NSLog("[BLEPeripheralService] Advertisement data: \(advertisementData)")
        peripheralManager.startAdvertising(advertisementData)
        isAdvertising = true
        lastError = nil
    }
    
    func stopAdvertising() {
        guard isAdvertising else { return }
        
        print("[BLEPeripheralService] Stopping advertising")
        peripheralManager.stopAdvertising()
        isAdvertising = false
    }
    
    func sendData(_ data: String, to central: CBCentral) {
        guard let data = data.data(using: .utf8) else { return }
        
        let success = peripheralManager.updateValue(
            data,
            for: readCharacteristic,
            onSubscribedCentrals: [central]
        )
        
        if success {
            print("[BLEPeripheralService] Sent data to central: \(data.count) bytes")
        } else {
            print("[BLEPeripheralService] Failed to send data to central")
        }
    }
}

// MARK: - CBPeripheralManagerDelegate
extension BLEPeripheralService: CBPeripheralManagerDelegate {
    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        DispatchQueue.main.async {
            print("[BLEPeripheralService] Peripheral manager state changed to: \(peripheral.state.rawValue)")
            self.isBluetoothEnabled = peripheral.state == .poweredOn
            
            switch peripheral.state {
            case .poweredOn:
                print("[BLEPeripheralService] Peripheral manager powered on, ready to advertise")
                NSLog("[BLEPeripheralService] Peripheral manager powered on, ready to advertise")
                self.lastError = nil
                // 自动开始广播
                print("[BLEPeripheralService] Auto-starting advertising")
                self.startAdvertising()
            case .poweredOff:
                print("[BLEPeripheralService] Peripheral manager powered off")
                NSLog("[BLEPeripheralService] Peripheral manager powered off")
                self.lastError = "蓝牙已关闭"
                self.stopAdvertising()
            case .unauthorized:
                print("[BLEPeripheralService] Peripheral manager unauthorized")
                NSLog("[BLEPeripheralService] Peripheral manager unauthorized")
                self.lastError = "蓝牙权限被拒绝"
            case .unsupported:
                print("[BLEPeripheralService] Peripheral manager unsupported")
                NSLog("[BLEPeripheralService] Peripheral manager unsupported")
                self.lastError = "设备不支持蓝牙"
            case .resetting:
                print("[BLEPeripheralService] Peripheral manager resetting")
                NSLog("[BLEPeripheralService] Peripheral manager resetting")
                self.lastError = "蓝牙正在重置"
            case .unknown:
                print("[BLEPeripheralService] Peripheral manager state unknown")
                NSLog("[BLEPeripheralService] Peripheral manager state unknown")
                self.lastError = "蓝牙状态未知"
            @unknown default:
                print("[BLEPeripheralService] Peripheral manager state unknown default")
                NSLog("[BLEPeripheralService] Peripheral manager state unknown default")
                self.lastError = "蓝牙状态未知"
            }
        }
    }
    
    func peripheralManager(_ peripheral: CBPeripheralManager, didAdd service: CBService, error: Error?) {
        if let error = error {
            print("[BLEPeripheralService] Failed to add service: \(error.localizedDescription)")
            DispatchQueue.main.async {
                self.lastError = "添加服务失败: \(error.localizedDescription)"
            }
        } else {
            print("[BLEPeripheralService] Service added successfully")
        }
    }
    
    func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        if let error = error {
            print("[BLEPeripheralService] Failed to start advertising: \(error.localizedDescription)")
            DispatchQueue.main.async {
                self.lastError = "开始广播失败: \(error.localizedDescription)"
                self.isAdvertising = false
            }
        } else {
            print("[BLEPeripheralService] Started advertising successfully")
        }
    }
    
    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didSubscribeTo characteristic: CBCharacteristic) {
        print("[BLEPeripheralService] Central subscribed to characteristic: \(central.identifier)")
        DispatchQueue.main.async {
            if !self.connectedCentrals.contains(central) {
                self.connectedCentrals.append(central)
            }
        }
    }
    
    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didUnsubscribeFrom characteristic: CBCharacteristic) {
        print("[BLEPeripheralService] Central unsubscribed from characteristic: \(central.identifier)")
        DispatchQueue.main.async {
            self.connectedCentrals.removeAll { $0.identifier == central.identifier }
        }
    }
    
    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveWrite requests: [CBATTRequest]) {
        for request in requests {
            if let data = request.value,
               let message = String(data: data, encoding: .utf8) {
                print("[BLEPeripheralService] Received data: \(message)")
                
                // 响应写入请求
                peripheral.respond(to: request, withResult: .success)
                
                // 这里可以处理接收到的剪贴板数据
                DispatchQueue.main.async {
                    // 更新本地剪贴板
                    NSPasteboard.general.clearContents()
                    NSPasteboard.general.setString(message, forType: .string)
                }
            }
        }
    }
}