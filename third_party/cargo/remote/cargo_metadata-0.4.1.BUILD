"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


rust_library(
    name = "cargo_metadata",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__error_chain__0_11_0//:error_chain",
        "@raze__semver__0_8_0//:semver",
        "@raze__serde__1_0_36//:serde",
        "@raze__serde_derive__1_0_36//:serde_derive",
        "@raze__serde_json__1_0_13//:serde_json",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "selftest" with type "test" omitted
