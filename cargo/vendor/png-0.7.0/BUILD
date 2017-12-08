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
    name = "png",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-0.7.0:bitflags",
        "//cargo/vendor/deflate-0.7.17:deflate",
        "//cargo/vendor/inflate-0.2.0:inflate",
        "//cargo/vendor/num-iter-0.1.34:num_iter",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
        "deflate",
        "png-encoding",
    ],
)
# Unsupported target "pngcheck" with type "example" omitted
# Unsupported target "show" with type "example" omitted
