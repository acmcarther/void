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
        "//cargo:lazy_static",
        "//cargo:log",
        "//cargo:vulkano",
        "//cargo:vulkano_win",
        "//cargo:winit",
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
