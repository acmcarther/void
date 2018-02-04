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

# Unsupported target "simple_client_proto" with type "test" omitted
# Unsupported target "simple_framed" with type "test" omitted
# Unsupported target "test_multiplex_client" with type "test" omitted
# Unsupported target "test_multiplex_deadlock" with type "test" omitted
# Unsupported target "test_multiplex_server" with type "test" omitted
# Unsupported target "test_pipeline_client" with type "test" omitted
# Unsupported target "test_pipeline_server" with type "test" omitted

rust_library(
    name = "tokio_proto",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__futures__0_1_17//:futures",
        "@raze__log__0_3_9//:log",
        "@raze__net2__0_2_31//:net2",
        "@raze__rand__0_3_20//:rand",
        "@raze__slab__0_3_0//:slab",
        "@raze__smallvec__0_2_1//:smallvec",
        "@raze__take__0_1_0//:take",
        "@raze__tokio_core__0_1_12//:tokio_core",
        "@raze__tokio_io__0_1_4//:tokio_io",
        "@raze__tokio_service__0_1_0//:tokio_service",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

