package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "spect_easy",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//rust/core/net/web/spect",
        "//rust/core/net/web/spect/spect_zcfg",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_binary(
    name = "demo",
    srcs = ["examples/demo.rs"],
    deps = [
        ":spect_easy",
        "//rust/core/base",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:zcfg",
    ],
)
