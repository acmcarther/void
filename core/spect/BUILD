package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "spect",
    srcs = ["spect.rs"],
    deps = [
        "//third_party/cargo:futures",
        "//third_party/cargo:hyper",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_library(
    name = "spect_zcfg",
    srcs = ["spect_zcfg.rs"],
    deps = [
        ":spect",
        "//third_party/cargo:futures",
        "//third_party/cargo:hyper",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_binary(
    name = "demo",
    srcs = ["demo.rs"],
    deps = [
        ":spect",
        ":spect_zcfg",
        "//core:init",
    ],
)
