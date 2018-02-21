package(default_visibility = [
    "//game/net:__subpackages__",
    "//experimental:__subpackages__"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "netcode_global",
    srcs = ["netcode_global.rs"],
    deps = [
        "//third_party/cargo:log",
        "//core:init",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

rust_library(
    name = "netcode_api",
    srcs = ["netcode_api.rs"],
    deps = [
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:zcfg",
        "@netcode_io//:netcode_io_sys",
    ],
)

rust_library(
    name = "netcode_client",
    srcs = ["netcode_client.rs"],
    deps = [
        ":netcode_api",
        ":netcode_terrible_constants",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:libc",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:zcfg_flag_parser",
        "//core:init",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

rust_library(
    name = "netcode_terrible_constants",
    srcs = ["netcode_terrible_constants.rs"],
    deps = [],
)

rust_library(
    name = "netcode_server",
    srcs = ["netcode_server.rs"],
    deps = [
        ":netcode_api",
        ":netcode_terrible_constants",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:libc",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:zcfg_flag_parser",
        "//core:init",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)


rust_binary(
    name = "netcode_demo",
    srcs = ["netcode_demo.rs"],
    deps = [
        "//core:init",
        ":netcode_server",
        ":netcode_client",
        ":netcode_api",
        ":netcode_global",
        "@netcode_io//:netcode_io_sys",
        "//third_party/cargo:log",
    ]
)

