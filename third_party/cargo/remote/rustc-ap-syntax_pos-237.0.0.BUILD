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
  name = "rustc_ap_syntax_pos",
  actual = ":syntax_pos",
)

rust_library(
    name = "syntax_pos",
    crate_root = "lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__cfg_if__0_1_5//:cfg_if",
        "@raze__rustc_ap_arena__237_0_0//:rustc_ap_arena",
        "@raze__rustc_ap_rustc_data_structures__237_0_0//:rustc_ap_rustc_data_structures",
        "@raze__rustc_ap_serialize__237_0_0//:rustc_ap_serialize",
        "@raze__scoped_tls__0_1_2//:scoped_tls",
        "@raze__unicode_width__0_1_5//:unicode_width",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "237.0.0",
    crate_features = [
    ],
)

