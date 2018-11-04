package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "demo",
    srcs = ["demo.rs"],
    deps = [
        "//rust/core/base",
        "//third_party/cargo:log",
        "//third_party/cargo:sdl2",
    ],
)
