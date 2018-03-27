package(default_visibility = [
    "//experimental:__subpackages__",
    "//game/net:__subpackages__",
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
        "//core:init",
        "//third_party/cargo:log",
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
        "//core:init",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:libc",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:zcfg_flag_parser",
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
        "//core:init",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:libc",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:zcfg_flag_parser",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

rust_binary(
    name = "netcode_demo",
    srcs = ["netcode_demo.rs"],
    deps = [
        ":netcode_api",
        ":netcode_client",
        ":netcode_global",
        ":netcode_server",
        "//core:init",
        "//third_party/cargo:log",
        "@netcode_io//:netcode_io_sys",
    ],
)
