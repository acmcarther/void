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


# Unsupported target "cat" with type "example" omitted
rust_library(
    name = "memmap",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/fs2-0.2.5:fs2",
        "//cargo/vendor/kernel32-sys-0.2.2:kernel32_sys",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/winapi-0.2.8:winapi",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
