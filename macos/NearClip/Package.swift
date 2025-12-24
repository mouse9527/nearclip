// swift-tools-version:5.9
import PackageDescription
import Foundation

// Get the directory containing Package.swift
let packageDir = URL(fileURLWithPath: #filePath).deletingLastPathComponent().path
let libraryPath = packageDir + "/../../target/swift"
let staticLibraryPath = libraryPath + "/libnearclip_ffi.a"

let package = Package(
    name: "NearClip",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .executable(name: "NearClip", targets: ["NearClip"])
    ],
    targets: [
        .executableTarget(
            name: "NearClip",
            dependencies: ["NearClipFFI"],
            path: "Sources/NearClip",
            exclude: ["Resources"],
            linkerSettings: [
                .linkedFramework("AppKit"),
                .linkedFramework("ServiceManagement"),
                .linkedFramework("Security"),
                .linkedFramework("CoreBluetooth"),
                // Force load the entire static library to ensure all symbols are included
                .unsafeFlags(["-Xlinker", "-force_load", "-Xlinker", staticLibraryPath])
            ]
        ),
        .systemLibrary(
            name: "NearClipFFI",
            path: "Sources/NearClipFFI"
        )
    ]
)
