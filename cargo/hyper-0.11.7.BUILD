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


rust_library(
    name = "hyper",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__base64__0_6_0//:base64",
        "@raze__bytes__0_4_5//:bytes",
        "@raze__futures__0_1_17//:futures",
        "@raze__futures_cpupool__0_1_7//:futures_cpupool",
        "@raze__httparse__1_2_3//:httparse",
        "@raze__language_tags__0_2_2//:language_tags",
        "@raze__log__0_3_8//:log",
        "@raze__mime__0_3_5//:mime",
        "@raze__percent_encoding__1_0_1//:percent_encoding",
        "@raze__relay__0_1_0//:relay",
        "@raze__time__0_1_38//:time",
        "@raze__tokio_core__0_1_10//:tokio_core",
        "@raze__tokio_io__0_1_4//:tokio_io",
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
    ],
)

