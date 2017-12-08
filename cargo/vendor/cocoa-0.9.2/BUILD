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
    name = "cocoa",
    crate_root = "src/lib.rs",
    crate_type = "rlib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-0.9.1:bitflags",
        "//cargo/vendor/block-0.1.6:block",
        "//cargo/vendor/core-graphics-0.8.2:core_graphics",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/objc-0.2.2:objc",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "foundation" with type "test" omitted
# Unsupported target "hello_world" with type "example" omitted
# Unsupported target "tab_view" with type "example" omitted
