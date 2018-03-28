"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "bounds_generation" with type "test" omitted
# Unsupported target "build_fn" with type "test" omitted
# Unsupported target "builder_name" with type "test" omitted
# Unsupported target "channel" with type "example" omitted
# Unsupported target "compiletests" with type "test" omitted
# Unsupported target "custom_default" with type "test" omitted
# Unsupported target "custom_defaults" with type "example" omitted
# Unsupported target "deny_missing_docs" with type "example" omitted

rust_library(
    name = "derive_builder",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__derive_builder_core__0_2_0//:derive_builder_core",
        "@raze__quote__0_3_15//:quote",
        "@raze__syn__0_11_11//:syn",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    data = glob(["src/doc_tpl/*"]),
    crate_features = [
    ],
)

# Unsupported target "derive_trait" with type "test" omitted
# Unsupported target "doc_example" with type "example" omitted
# Unsupported target "generic_structs" with type "test" omitted
# Unsupported target "lifetime" with type "test" omitted
# Unsupported target "readme_example" with type "example" omitted
# Unsupported target "setter_into" with type "test" omitted
# Unsupported target "setter_name" with type "test" omitted
# Unsupported target "setter_pattern" with type "test" omitted
# Unsupported target "setter_prefix" with type "test" omitted
# Unsupported target "setter_visibility" with type "test" omitted
# Unsupported target "skeptic" with type "test" omitted
# Unsupported target "skip-setter" with type "test" omitted
# Unsupported target "try_setter" with type "test" omitted
# Unsupported target "validation" with type "example" omitted
# Unsupported target "validation" with type "test" omitted
