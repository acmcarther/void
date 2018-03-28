"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


rust_library(
    name = "clang_sys",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__glob__0_2_11//:glob",
        "@raze__libc__0_2_40//:libc",
        "@//third_party/cargo/overrides/libloading-0.5.0:libloading",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "clang_3_9",
        "gte_clang_3_6",
        "gte_clang_3_7",
        "gte_clang_3_8",
        "gte_clang_3_9",
        "libloading",
        "runtime",
    ],
)

# Unsupported target "lib" with type "test" omitted
