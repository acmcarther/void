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
    name = "num_rational",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/num-bigint-0.1.41:num_bigint",
        "//cargo/vendor/num-integer-0.1.35:num_integer",
        "//cargo/vendor/num-traits-0.1.41:num_traits",
        "//cargo/vendor/rustc-serialize-0.3.24:rustc_serialize",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "bigint",
        "default",
        "num-bigint",
        "rustc-serialize",
    ],
)
