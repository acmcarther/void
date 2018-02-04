"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

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
        "@raze__encoding_index_japanese__1_20141219_5//:encoding_index_japanese",
        "@raze__encoding_index_korean__1_20141219_5//:encoding_index_korean",
        "@raze__encoding_index_simpchinese__1_20141219_5//:encoding_index_simpchinese",
        "@raze__encoding_index_singlebyte__1_20141219_5//:encoding_index_singlebyte",
        "@raze__encoding_index_tradchinese__1_20141219_5//:encoding_index_tradchinese",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "recode" with type "example" omitted
