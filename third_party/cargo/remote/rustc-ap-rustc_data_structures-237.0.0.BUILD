"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//third_party/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
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
        "@raze__cfg_if__0_1_5//:cfg_if",
        "@raze__ena__0_9_3//:ena",
        "@raze__log__0_4_5//:log",
        "@raze__parking_lot__0_5_5//:parking_lot",
        "@raze__parking_lot_core__0_2_14//:parking_lot_core",
        "@raze__rustc_ap_rustc_cratesio_shim__237_0_0//:rustc_ap_rustc_cratesio_shim",
        "@raze__rustc_ap_serialize__237_0_0//:rustc_ap_serialize",
        "@raze__rustc_hash__1_0_1//:rustc_hash",
        "@raze__rustc_rayon__0_1_1//:rustc_rayon",
        "@raze__rustc_rayon_core__0_1_1//:rustc_rayon_core",
        "@raze__smallvec__0_6_5//:smallvec",
        "@raze__stable_deref_trait__1_1_1//:stable_deref_trait",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "237.0.0",
    crate_features = [
    ],
)

