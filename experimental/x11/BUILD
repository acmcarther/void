package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "x11",
    srcs = ["x11.rs"],
    deps = [
        "//core:init",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:x11_dl",
        "//third_party/cargo:zcfg",
    ],
)
