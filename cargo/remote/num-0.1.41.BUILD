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

# Unsupported target "bigint" with type "bench" omitted

rust_library(
    name = "num",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__num_bigint__0_1_41//:num_bigint",
        "@raze__num_complex__0_1_41//:num_complex",
        "@raze__num_integer__0_1_35//:num_integer",
        "@raze__num_iter__0_1_34//:num_iter",
        "@raze__num_rational__0_1_40//:num_rational",
        "@raze__num_traits__0_1_41//:num_traits",
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
