package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "srv_physics",
    srcs = ["srv_physics.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
        "//cargo/vendor:specs",
        "//common:service_spec",
    ],
)
