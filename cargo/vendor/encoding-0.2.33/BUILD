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
    name = "encoding",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/encoding-index-japanese-1.20141219.5:encoding_index_japanese",
        "//cargo/vendor/encoding-index-korean-1.20141219.5:encoding_index_korean",
        "//cargo/vendor/encoding-index-simpchinese-1.20141219.5:encoding_index_simpchinese",
        "//cargo/vendor/encoding-index-singlebyte-1.20141219.5:encoding_index_singlebyte",
        "//cargo/vendor/encoding-index-tradchinese-1.20141219.5:encoding_index_tradchinese",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "recode" with type "example" omitted
