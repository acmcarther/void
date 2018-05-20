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
        "//third_party/cargo:chrono",
        "//third_party/cargo:fern",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:zcfg_flag_parser",
    ],
)

rust_library(
    name = "task",
    srcs = ["task.rs"],
    deps = [
        "//third_party/cargo:log",
    ],
)
