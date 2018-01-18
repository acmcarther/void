"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//cargo:__subpackages__"])

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
        "//cargo/vendor/glob-0.2.11:glob",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/libloading-0.4.3:libloading",
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
