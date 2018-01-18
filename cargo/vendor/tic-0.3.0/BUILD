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
# Unsupported target "allan" with type "example" omitted
# Unsupported target "benchmark" with type "example" omitted

rust_library(
    name = "tic",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/allan-0.2.4:allan",
        "//cargo/vendor/clocksource-0.2.3:clocksource",
        "//cargo/vendor/fnv-1.0.6:fnv",
        "//cargo/vendor/getopts-0.2.15:getopts",
        "//cargo/vendor/heatmap-0.6.6:heatmap",
        "//cargo/vendor/histogram-0.6.8:histogram",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/mio-0.6.11:mio",
        "//cargo/vendor/mpmc-0.1.5:mpmc",
        "//cargo/vendor/time-0.1.38:time",
        "//cargo/vendor/tiny_http-0.5.8:tiny_http",
        "//cargo/vendor/waterfall-0.7.1:waterfall",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
    ],
)

