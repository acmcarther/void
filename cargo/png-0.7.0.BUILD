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
    name = "png",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__0_7_0//:bitflags",
        "@raze__deflate__0_7_17//:deflate",
        "@raze__inflate__0_2_0//:inflate",
        "@raze__num_iter__0_1_34//:num_iter",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "deflate",
        "png-encoding",
    ],
)

# Unsupported target "pngcheck" with type "example" omitted
# Unsupported target "show" with type "example" omitted
