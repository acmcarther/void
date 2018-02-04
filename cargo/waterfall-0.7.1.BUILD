"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


rust_library(
    name = "waterfall",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__heatmap__0_6_6//:heatmap",
        "@raze__hsl__0_1_1//:hsl",
        "@raze__png__0_7_0//:png",
        "@raze__rusttype__0_1_2//:rusttype",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

