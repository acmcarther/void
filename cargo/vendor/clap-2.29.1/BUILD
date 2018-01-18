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
    name = "clap",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/ansi_term-0.10.2:ansi_term",
        "//cargo/vendor/atty-0.2.6:atty",
        "//cargo/vendor/bitflags-1.0.1:bitflags",
        "//cargo/vendor/strsim-0.6.0:strsim",
        "//cargo/vendor/textwrap-0.9.0:textwrap",
        "//cargo/vendor/unicode-width-0.1.4:unicode_width",
        "//cargo/vendor/vec_map-0.8.0:vec_map",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "ansi_term",
        "atty",
        "color",
        "default",
        "strsim",
        "suggestions",
        "vec_map",
    ],
)

