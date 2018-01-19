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
        "//cargo:chrono",
        "//cargo:log",
        "//cargo:specs",
        "//common:service_spec",
    ],
)
