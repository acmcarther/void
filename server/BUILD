package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
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
