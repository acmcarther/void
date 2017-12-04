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


# Unsupported target "cmd-program" with type "example" omitted
rust_library(
    name = "fern",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/log-0.3.8:log",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "lib" with type "test" omitted
