package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "service_specification",
    srcs = ["service_specification.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
    ],
)

rust_library(
    name = "physics",
    srcs = ["physics.rs"],
    deps = [
        ":service_specification",
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
    ],
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        ":physics",
        ":service_specification",
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
    ],
)

rust_binary(
    name = "launcher",
    srcs = ["launcher.rs"],
    deps = [
        ":server",
        "//common:init",
    ],
)
