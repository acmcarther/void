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

# Unsupported target "functions" with type "test" omitted

rust_library(
    name = "libloading",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__lazy_static__1_0_0//:lazy_static",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "markers" with type "test" omitted
# Unsupported target "windows" with type "test" omitted
