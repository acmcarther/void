package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "icosphere",
    srcs = ["icosphere.rs"],
    deps = [
        "//core/gfx:gfx_basics",
        "//core/math:vector",
        "//third_party/cargo:itertools",
    ],
)

rust_test(
    name = "icosphere_test",
    deps = [":icosphere"],
)
