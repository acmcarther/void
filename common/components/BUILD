package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "com_physics",
    srcs = ["com_physics.rs"],
    deps = [
        "//cargo/vendor:specs",
    ],
)
