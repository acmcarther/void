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
  name = "rustc_ap_rustc_target",
  actual = ":rustc_target",
)

rust_library(
    name = "rustc_target",
    crate_root = "lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__1_0_4//:bitflags",
        "@raze__log__0_4_5//:log",
        "@raze__rustc_ap_rustc_cratesio_shim__237_0_0//:rustc_ap_rustc_cratesio_shim",
        "@raze__rustc_ap_serialize__237_0_0//:rustc_ap_serialize",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "237.0.0",
    crate_features = [
    ],
)

