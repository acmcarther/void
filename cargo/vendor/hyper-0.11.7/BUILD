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

rust_library(
    name = "hyper",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/base64-0.6.0:base64",
        "//cargo/vendor/bytes-0.4.5:bytes",
        "//cargo/vendor/futures-0.1.17:futures",
        "//cargo/vendor/futures-cpupool-0.1.7:futures_cpupool",
        "//cargo/vendor/httparse-1.2.3:httparse",
        "//cargo/vendor/language-tags-0.2.2:language_tags",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/mime-0.3.5:mime",
        "//cargo/vendor/percent-encoding-1.0.1:percent_encoding",
        "//cargo/vendor/relay-0.1.0:relay",
        "//cargo/vendor/time-0.1.38:time",
        "//cargo/vendor/tokio-core-0.1.10:tokio_core",
        "//cargo/vendor/tokio-io-0.1.4:tokio_io",
        "//cargo/vendor/tokio-proto-0.1.1:tokio_proto",
        "//cargo/vendor/tokio-service-0.1.0:tokio_service",
        "//cargo/vendor/unicase-2.1.0:unicase",
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

