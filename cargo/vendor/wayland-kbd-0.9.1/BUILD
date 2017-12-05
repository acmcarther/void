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


# Unsupported target "basic_input" with type "example" omitted
rust_library(
    name = "wayland_kbd",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-0.7.0:bitflags",
        "//cargo/vendor/dlib-0.3.1:dlib",
        "//cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//cargo/vendor/memmap-0.4.0:memmap",
        "//cargo/vendor/wayland-client-0.9.10:wayland_client",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
