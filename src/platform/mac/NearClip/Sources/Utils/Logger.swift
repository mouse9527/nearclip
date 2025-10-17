import Foundation
import os.log

// 日志级别
enum LogLevel: String, CaseIterable {
    case debug = "DEBUG"
    case info = "INFO"
    case warning = "WARNING"
    case error = "ERROR"
}

// 简单的日志工具
class Logger {
    static let shared = Logger()

    private let osLog = OSLog(subsystem: "com.nearclip.mac", category: "NearClip")
    private let enableDebugLogging: Bool

    private init() {
        #if DEBUG
        enableDebugLogging = true
        #else
        enableDebugLogging = false
        #endif
    }

    // 调试日志
    func debug(_ message: String, _ args: CVarArg...) {
        guard enableDebugLogging else { return }
        log(message: message, level: .debug, args: args)
    }

    // 信息日志
    func info(_ message: String, _ args: CVarArg...) {
        log(message: message, level: .info, args: args)
    }

    // 警告日志
    func warning(_ message: String, _ args: CVarArg...) {
        log(message: message, level: .warning, args: args)
    }

    // 错误日志
    func error(_ message: String, _ args: CVarArg...) {
        log(message: message, level: .error, args: args)
    }

    private func log(message: String, level: LogLevel, args: CVarArg...) {
        let formattedMessage = String(format: message, arguments: args)
        let timestamp = DateFormatter.timestamp.string(from: Date())
        let logMessage = "[\(timestamp)] [\(level.rawValue)] \(formattedMessage)"

        // 输出到控制台
        print(logMessage)

        // 输出到系统日志
        let osLogType: OSLogType
        switch level {
        case .debug:
            osLogType = .debug
        case .info:
            osLogType = .info
        case .warning:
            osLogType = .default
        case .error:
            osLogType = .error
        }

        os_log("%{public}@", log: osLog, type: osLogType, formattedMessage)
    }
}

// 时间戳格式化器扩展
private extension DateFormatter {
    static let timestamp: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd HH:mm:ss.SSS"
        return formatter
    }()
}