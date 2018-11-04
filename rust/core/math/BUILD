package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_library(
    name = "math",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//third_party/cargo:log",
    ],
)

rust_test(
    name = "math_test",
    deps = [
        ":math",
    ],
)
