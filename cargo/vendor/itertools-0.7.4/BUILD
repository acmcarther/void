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


# Unsupported target "bench1" with type "bench" omitted
# Unsupported target "iris" with type "example" omitted
rust_library(
    name = "itertools",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/either-1.4.0:either",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        "default",
        "use_std",
    ],
)
# Unsupported target "merge_join" with type "test" omitted
# Unsupported target "peeking_take_while" with type "test" omitted
# Unsupported target "quick" with type "test" omitted
# Unsupported target "test_core" with type "test" omitted
# Unsupported target "test_std" with type "test" omitted
# Unsupported target "tuple_combinations" with type "bench" omitted
# Unsupported target "tuples" with type "bench" omitted
# Unsupported target "tuples" with type "test" omitted
# Unsupported target "zip" with type "test" omitted
