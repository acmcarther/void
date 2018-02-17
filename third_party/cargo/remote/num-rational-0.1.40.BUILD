"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT,Apache-2.0"
])

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
        "@raze__num_bigint__0_1_41//:num_bigint",
        "@raze__num_integer__0_1_35//:num_integer",
        "@raze__num_traits__0_1_41//:num_traits",
        "@raze__rustc_serialize__0_3_24//:rustc_serialize",
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

