package(default_visibility = [
    "//interconnect:__subpackages__"
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
        "//cargo:log",
        "//init:init",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

rust_library(
    name = "netcode_api",
    srcs = ["netcode_api.rs"],
    deps = [
        "//cargo:lazy_static",
        "//cargo:zcfg",
        "@netcode_io//:netcode_io_sys",
    ],
)

rust_library(
    name = "netcode_client",
    srcs = ["netcode_client.rs"],
    deps = [
        ":netcode_api",
        ":netcode_terrible_constants",
        "//cargo:lazy_static",
        "//cargo:log",
        "//cargo:libc",
        "//cargo:zcfg",
        "//cargo:zcfg_flag_parser",
        "//init:init",
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
        "//cargo:lazy_static",
        "//cargo:log",
        "//cargo:libc",
        "//cargo:zcfg",
        "//cargo:zcfg_flag_parser",
        "//init:init",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)


rust_binary(
    name = "netcode_demo",
    srcs = ["netcode_demo.rs"],
    deps = [
        "//init:init",
        ":netcode_server",
        ":netcode_client",
        ":netcode_api",
        ":netcode_global",
        "@netcode_io//:netcode_io_sys",
        "//cargo:log",
    ]
)

