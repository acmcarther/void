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

# Unsupported target "dump-syntax" with type "example" omitted

rust_library(
    name = "syn",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__proc_macro2__0_3_8//:proc_macro2",
        "@raze__quote__0_5_2//:quote",
        "@raze__unicode_xid__0_1_0//:unicode_xid",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "clone-impls",
        "default",
        "derive",
        "parsing",
        "printing",
        "proc-macro",
        "proc-macro2",
        "quote",
    ],
)

