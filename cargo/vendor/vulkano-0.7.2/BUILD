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
    name = "vulkano",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/crossbeam-0.3.0:crossbeam",
        "//cargo/vendor/fnv-1.0.6:fnv",
        "//cargo/vendor/half-1.0.1:half",
        "//cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//cargo/vendor/shared_library-0.1.7:shared_library",
        "//cargo/vendor/smallvec-0.4.4:smallvec",
        "//cargo/vendor/vk-sys-0.3.2:vk_sys",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
