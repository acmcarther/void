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


rust_binary(
    name = "bench",
    crate_root = "src/bin/bench.rs",
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
rust_library(
    name = "crossbeam",
    crate_root = "src/lib.rs",
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
rust_binary(
    name = "stress_msq",
    crate_root = "src/bin/stress-msq.rs",
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
