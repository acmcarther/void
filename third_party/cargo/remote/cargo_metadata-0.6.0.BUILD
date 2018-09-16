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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "cargo_metadata",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__error_chain__0_12_0//:error_chain",
        "@raze__semver__0_9_0//:semver",
        "@raze__serde__1_0_79//:serde",
        "@raze__serde_derive__1_0_79//:serde_derive",
        "@raze__serde_json__1_0_27//:serde_json",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.6.0",
    crate_features = [
        "backtrace",
        "default",
        "error-chain",
    ],
)

# Unsupported target "selftest" with type "test" omitted
