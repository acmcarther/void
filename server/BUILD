package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
        "//common:service_spec",
        "//common/service:srv_physics",
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

rust_library(
    name = "space",
    srcs = ["space.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
    ],
)

rust_test(
    name = "space_test",
    deps = [
        ":space",
    ],
)
