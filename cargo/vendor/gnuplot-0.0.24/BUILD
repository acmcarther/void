"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//cargo:__subpackages__"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)
# Unsupported target "animation_example" with type "example" omitted
# Unsupported target "example1" with type "example" omitted
# Unsupported target "example2" with type "example" omitted
# Unsupported target "example3" with type "example" omitted
# Unsupported target "example4" with type "example" omitted

rust_library(
    name = "gnuplot",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

