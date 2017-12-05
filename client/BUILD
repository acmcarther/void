package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "client",
    srcs = ["client.rs"],
    deps = [
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:vulkano",
        "//cargo/vendor:vulkano_win",
        "//cargo/vendor:winit",
    ],
)

rust_binary(
    name = "launcher",
    srcs = ["launcher.rs"],
    deps = [
        ":client",
        "//common:init",
    ],
)
