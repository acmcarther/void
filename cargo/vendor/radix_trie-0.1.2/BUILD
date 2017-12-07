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


# Unsupported target "child_iter" with type "example" omitted
# Unsupported target "debug" with type "example" omitted
# Unsupported target "opt" with type "example" omitted
rust_library(
    name = "radix_trie",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/endian-type-0.1.2:endian_type",
        "//cargo/vendor/nibble_vec-0.0.3:nibble_vec",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "string_frequency" with type "example" omitted
