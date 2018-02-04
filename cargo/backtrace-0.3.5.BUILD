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

# Unsupported target "backtrace" with type "example" omitted

rust_library(
    name = "backtrace",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__backtrace_sys__0_1_16//:backtrace_sys",
        "@raze__cfg_if__0_1_2//:cfg_if",
        "@raze__libc__0_2_36//:libc",
        "@raze__rustc_demangle__0_1_5//:rustc_demangle",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "backtrace-sys",
        "coresymbolication",
        "dbghelp",
        "default",
        "dladdr",
        "libbacktrace",
        "libunwind",
        "winapi",
    ],
)

# Unsupported target "long_fn_name" with type "test" omitted
# Unsupported target "raw" with type "example" omitted
# Unsupported target "smoke" with type "test" omitted
