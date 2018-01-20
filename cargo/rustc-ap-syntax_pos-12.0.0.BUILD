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

alias(
  name = "rustc_ap_syntax_pos",
  actual = ":syntax_pos",
)

rust_library(
    name = "syntax_pos",
    crate_root = "lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__rustc_ap_rustc_data_structures__12_0_0//:rustc_ap_rustc_data_structures",
        "@raze__rustc_ap_serialize__12_0_0//:rustc_ap_serialize",
        "@raze__term__0_4_6//:term",
        "@raze__unicode_width__0_1_4//:unicode_width",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

