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
# Unsupported target "arithmetic" with type "test" omitted
# Unsupported target "arithmetic_ast" with type "test" omitted
# Unsupported target "blockbuf-arithmetic" with type "test" omitted
# Unsupported target "cross_function_backtracking" with type "test" omitted
# Unsupported target "float" with type "test" omitted
# Unsupported target "ini" with type "test" omitted
# Unsupported target "ini_str" with type "test" omitted
# Unsupported target "issues" with type "test" omitted
# Unsupported target "json" with type "test" omitted
# Unsupported target "mp4" with type "test" omitted
# Unsupported target "multiline" with type "test" omitted
# Unsupported target "named_args" with type "test" omitted

rust_library(
    name = "nom",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/memchr-1.0.2:memchr",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "memchr",
        "std",
        "stream",
        "verbose-errors",
    ],
)

# Unsupported target "omnom" with type "test" omitted
# Unsupported target "overflow" with type "test" omitted
# Unsupported target "reborrow_fold" with type "test" omitted
# Unsupported target "test1" with type "test" omitted
