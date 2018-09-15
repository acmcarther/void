package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_library(
    name = "math_basics",
    srcs = ["math_basics.rs"],
    deps = [
    ],
)

rust_library(
    name = "vector",
    srcs = ["vector.rs"],
    deps = [
        ":math_basics",
        "//third_party/cargo:log",
    ],
)

rust_test(
    name = "vector_test",
    deps = [
        ":vector",
    ],
)
