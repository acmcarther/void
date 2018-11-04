package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "util",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//third_party/cargo:log",
    ],
)
