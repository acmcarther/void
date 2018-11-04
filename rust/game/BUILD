package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
)

rust_binary(
    name = "launcher",
    srcs = ["launcher_bin.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":game_client",
        ":game_server",
        "//core:init",
        "//core/util:task",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:zcfg",
    ],
)

rust_library(
    name = "game_client",
    srcs = ["game_client.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//core/util:task",
        "//third_party/cargo:log",
    ],
)

rust_library(
    name = "game_server",
    srcs = ["game_server.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//core/util:task",
        "//third_party/cargo:log",
    ],
)
