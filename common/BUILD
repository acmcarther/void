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
        "//cargo:chrono",
        "//cargo:fern",
        "//cargo:lazy_static",
        "//cargo:log",
        "//cargo:zcfg",
        "//cargo:zcfg_flag_parser",
    ],
)

rust_library(
    name = "service_spec",
    srcs = ["service_spec.rs"],
    deps = [
        "//cargo:chrono",
        "//cargo:log",
    ],
)
