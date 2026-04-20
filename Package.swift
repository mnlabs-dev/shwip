// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "shwip",
    platforms: [
        .macOS(.v14)
    ],
    targets: [
        .executableTarget(
            name: "shwip",
            path: "Sources/shwip"
        ),
        .testTarget(
            name: "shwipTests",
            dependencies: ["shwip"],
            path: "Tests/shwipTests"
        ),
    ]
)
