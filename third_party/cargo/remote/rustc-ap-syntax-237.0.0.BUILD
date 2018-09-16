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


# Unsupported target "build-script-build" with type "custom-build" omitted
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
        "@raze__bitflags__1_0_4//:bitflags",
        "@raze__log__0_4_5//:log",
        "@raze__rustc_ap_rustc_data_structures__237_0_0//:rustc_ap_rustc_data_structures",
        "@raze__rustc_ap_rustc_errors__237_0_0//:rustc_ap_rustc_errors",
        "@raze__rustc_ap_rustc_target__237_0_0//:rustc_ap_rustc_target",
        "@raze__rustc_ap_serialize__237_0_0//:rustc_ap_serialize",
        "@raze__rustc_ap_syntax_pos__237_0_0//:rustc_ap_syntax_pos",
        "@raze__scoped_tls__0_1_2//:scoped_tls",
        "@raze__smallvec__0_6_5//:smallvec",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "237.0.0",
    crate_features = [
    ],
)

