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
# Unsupported target "async_read" with type "test" omitted
# Unsupported target "codecs" with type "test" omitted
# Unsupported target "framed" with type "test" omitted
# Unsupported target "framed_read" with type "test" omitted
# Unsupported target "framed_write" with type "test" omitted
# Unsupported target "length_delimited" with type "test" omitted

rust_library(
    name = "tokio_io",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bytes-0.4.5:bytes",
        "//cargo/vendor/futures-0.1.17:futures",
        "//cargo/vendor/log-0.3.8:log",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

