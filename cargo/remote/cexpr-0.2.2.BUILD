"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


rust_library(
    name = "cexpr",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__nom__3_2_1//:nom",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "clang" with type "test" omitted
