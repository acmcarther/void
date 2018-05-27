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

alias(
  name = "rustc_ap_syntax",
  actual = ":syntax",
)

rust_library(
    name = "syntax",
    crate_root = "lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__1_0_3//:bitflags",
        "@raze__log__0_4_1//:log",
        "@raze__rustc_ap_rustc_cratesio_shim__29_0_0//:rustc_ap_rustc_cratesio_shim",
        "@raze__rustc_ap_rustc_data_structures__29_0_0//:rustc_ap_rustc_data_structures",
        "@raze__rustc_ap_rustc_errors__29_0_0//:rustc_ap_rustc_errors",
        "@raze__rustc_ap_serialize__29_0_0//:rustc_ap_serialize",
        "@raze__rustc_ap_syntax_pos__29_0_0//:rustc_ap_syntax_pos",
        "@raze__term__0_4_6//:term",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

