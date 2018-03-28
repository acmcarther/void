"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "basic" with type "bench" omitted
# Unsupported target "depth" with type "bench" omitted
# Unsupported target "depth" with type "example" omitted
# Unsupported target "hello" with type "example" omitted
# Unsupported target "smoke" with type "example" omitted
# Unsupported target "threadpool" with type "test" omitted

rust_library(
    name = "tokio_threadpool",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__crossbeam_deque__0_3_0//:crossbeam_deque",
        "@raze__futures__0_1_20//:futures",
        "@raze__log__0_3_9//:log",
        "@raze__num_cpus__1_8_0//:num_cpus",
        "@raze__rand__0_4_2//:rand",
        "@raze__tokio_executor__0_1_1//:tokio_executor",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
    ],
)

