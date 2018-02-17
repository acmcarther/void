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
    name = "png",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__1_0_1//:bitflags",
        "@raze__deflate__0_7_17//:deflate",
        "@raze__inflate__0_3_3//:inflate",
        "@raze__num_iter__0_1_34//:num_iter",
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
