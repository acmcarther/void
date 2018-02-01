package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "icosphere",
    srcs = ["icosphere.rs"],
    deps = [
        "//cargo:itertools",
    ],
)
