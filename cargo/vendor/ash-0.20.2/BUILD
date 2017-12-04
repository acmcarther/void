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


rust_library(
    name = "ash",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/shared_library-0.1.7:shared_library",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
    ],
)
