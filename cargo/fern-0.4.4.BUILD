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

# Unsupported target "cmd-program" with type "example" omitted
# Unsupported target "colored" with type "example" omitted

rust_library(
    name = "fern",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__log__0_3_9//:log",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "lib" with type "test" omitted
