"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "backcompat" with type "test" omitted
# Unsupported target "datetime" with type "test" omitted
# Unsupported target "decode" with type "example" omitted
# Unsupported target "display" with type "test" omitted
# Unsupported target "display-tricky" with type "test" omitted
# Unsupported target "formatting" with type "test" omitted
# Unsupported target "invalid" with type "test" omitted
# Unsupported target "invalid-encoder-misc" with type "test" omitted
# Unsupported target "invalid-misc" with type "test" omitted
# Unsupported target "parser" with type "test" omitted
# Unsupported target "pretty" with type "test" omitted
# Unsupported target "serde" with type "test" omitted
# Unsupported target "tables-last" with type "test" omitted

rust_library(
    name = "toml",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__serde__1_0_27//:serde",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "toml2json" with type "example" omitted
# Unsupported target "valid" with type "test" omitted
