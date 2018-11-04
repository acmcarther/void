package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "server",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//rust/core/net/low/common",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:libc",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:zcfg_flag_parser",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)
