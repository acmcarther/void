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


# Unsupported target "bigint" with type "bench" omitted
rust_library(
    name = "num",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/num-bigint-0.1.41:num_bigint",
        "//cargo/vendor/num-complex-0.1.41:num_complex",
        "//cargo/vendor/num-integer-0.1.35:num_integer",
        "//cargo/vendor/num-iter-0.1.34:num_iter",
        "//cargo/vendor/num-rational-0.1.40:num_rational",
        "//cargo/vendor/num-traits-0.1.41:num_traits",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "bigint",
        "complex",
        "default",
        "num-bigint",
        "num-complex",
        "num-rational",
        "rational",
        "rustc-serialize",
    ],
)
# Unsupported target "shootout-pidigits" with type "bench" omitted
