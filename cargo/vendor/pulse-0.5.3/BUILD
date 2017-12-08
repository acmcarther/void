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


# Unsupported target "barrier" with type "test" omitted
# Unsupported target "benches" with type "bench" omitted
rust_library(
    name = "pulse",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/atom-0.3.4:atom",
        "//cargo/vendor/time-0.1.38:time",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
    ],
)
# Unsupported target "select" with type "test" omitted
# Unsupported target "simple" with type "test" omitted
