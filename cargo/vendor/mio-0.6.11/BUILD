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


# Unsupported target "bench_poll" with type "bench" omitted
rust_library(
    name = "mio",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/iovec-0.1.1:iovec",
        "//cargo/vendor/lazycell-0.5.1:lazycell",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/net2-0.2.31:net2",
        "//cargo/vendor/slab-0.3.0:slab",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
        "with-deprecated",
    ],
)
# Unsupported target "test" with type "test" omitted
