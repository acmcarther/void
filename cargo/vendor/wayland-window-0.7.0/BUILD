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


# Unsupported target "simple_window" with type "example" omitted
rust_library(
    name = "wayland_window",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/byteorder-1.2.1:byteorder",
        "//cargo/vendor/tempfile-2.2.0:tempfile",
        "//cargo/vendor/wayland-client-0.9.10:wayland_client",
        "//cargo/vendor/wayland-protocols-0.9.10:wayland_protocols",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
