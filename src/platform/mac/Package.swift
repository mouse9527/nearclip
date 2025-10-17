// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "NearClip",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(
            name: "NearClip",
            targets: ["NearClip"]
        ),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.25.0"),
    ],
    targets: [
        .executableTarget(
            name: "NearClip",
            dependencies: [
                .product(name: "SwiftProtobuf", package: "swift-protobuf")
            ],
            path: "NearClip/Sources"
        ),
        .testTarget(
            name: "NearClipTests",
            dependencies: [
                "NearClip"
            ],
            path: "NearClip/Tests/NearClipTests"
        ),
    ]
)