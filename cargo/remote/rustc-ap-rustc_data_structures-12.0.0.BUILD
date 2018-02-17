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
  name = "rustc_ap_rustc_data_structures",
  actual = ":rustc_data_structures",
)

rust_library(
    name = "rustc_data_structures",
    crate_root = "lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__cfg_if__0_1_2//:cfg_if",
        "@raze__log__0_4_1//:log",
        "@raze__parking_lot__0_5_3//:parking_lot",
        "@raze__parking_lot_core__0_2_10//:parking_lot_core",
        "@raze__rustc_ap_serialize__12_0_0//:rustc_ap_serialize",
        "@raze__stable_deref_trait__1_0_0//:stable_deref_trait",
        "@raze__term__0_4_6//:term",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

