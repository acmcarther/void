package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "init",
    srcs = ["init.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:fern",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:zcfg",
        "//cargo/vendor:zcfg_flag_parser",
    ],
)
