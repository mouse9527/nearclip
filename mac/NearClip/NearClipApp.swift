//
//  NearClipApp.swift
//  NearClip
//
//  NearClip macOS 应用程序入口点
//

import SwiftUI
import OSLog
import CoreBluetooth

@main
struct NearClipApp: App {
    // 核心服务实例
    @StateObject private var nearClipCore = NearClipCore()

    // 系统日志
    private let logger = Logger(subsystem: "com.nearclip.NearClip", category: "App")

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(nearClipCore)
                .onAppear {
                    setupApplication()
                }
                .onDisappear {
                    cleanupApplication()
                }
        }
        .windowStyle(.hiddenTitleBar)
        .windowResizability(.contentSize)
    }

    // MARK: - 应用程序设置

    private func setupApplication() {
        logger.info("NearClip application starting...")

        Task {
            do {
                // 初始化核心服务
                try await nearClipCore.initialize()

                // 启动服务
                try await nearClipCore.start()

                logger.info("NearClip application started successfully")
            } catch {
                logger.error("Failed to start NearClip application: \(error.localizedDescription)")

                // 显示错误对话框
                await MainActor.run {
                    showErrorAlert(error: error)
                }
            }
        }
    }

    private func cleanupApplication() {
        logger.info("NearClip application terminating...")

        Task {
            do {
                try await nearClipCore.stop()
                logger.info("NearClip application stopped successfully")
            } catch {
                logger.error("Failed to stop NearClip application: \(error.localizedDescription)")
            }
        }
    }

    // MARK: - 错误处理

    private func showErrorAlert(error: Error) {
        // TODO: 实现错误对话框显示
        logger.critical("Critical error during startup: \(error.localizedDescription)")
    }
}