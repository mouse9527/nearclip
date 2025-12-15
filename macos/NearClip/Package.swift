// swift-tools-version:5.9
import PackageDescription

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
                .linkedLibrary("nearclip_ffi"),
                .unsafeFlags(["-L../../target/swift"])
            ]
        ),
        .systemLibrary(
            name: "NearClipFFI",
            path: "Sources/NearClipFFI"
        )
    ]
)
