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


rust_library(
    name = "hyper",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__base64__0_9_0//:base64",
        "@raze__bytes__0_4_6//:bytes",
        "@raze__futures__0_1_20//:futures",
        "@raze__futures_cpupool__0_1_8//:futures_cpupool",
        "@raze__httparse__1_2_4//:httparse",
        "@raze__iovec__0_1_2//:iovec",
        "@raze__language_tags__0_2_2//:language_tags",
        "@raze__log__0_4_1//:log",
        "@raze__mime__0_3_5//:mime",
        "@raze__percent_encoding__1_0_1//:percent_encoding",
        "@raze__relay__0_1_1//:relay",
        "@raze__time__0_1_39//:time",
        "@raze__tokio_core__0_1_16//:tokio_core",
        "@raze__tokio_io__0_1_6//:tokio_io",
        "@raze__tokio_proto__0_1_1//:tokio_proto",
        "@raze__tokio_service__0_1_0//:tokio_service",
        "@raze__unicase__2_1_0//:unicase",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "server-proto",
        "tokio-proto",
    ],
)

