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


# Unsupported target "caps" with type "example" omitted
# Unsupported target "indirect" with type "example" omitted
# Unsupported target "library" with type "example" omitted
rust_library(
    name = "metal_rs",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-0.9.1:bitflags",
        "//cargo/vendor/block-0.1.6:block",
        "//cargo/vendor/cocoa-0.9.2:cocoa",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/objc-0.2.2:objc",
        "//cargo/vendor/objc-foundation-0.1.1:objc_foundation",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "reflection" with type "example" omitted
# Unsupported target "window" with type "example" omitted
