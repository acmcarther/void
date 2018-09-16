"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//third_party/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "backtrace" with type "example" omitted

rust_library(
    name = "backtrace",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__backtrace_sys__0_1_24//:backtrace_sys",
        "@raze__cfg_if__0_1_5//:cfg_if",
        "@raze__libc__0_2_43//:libc",
        "@raze__rustc_demangle__0_1_9//:rustc_demangle",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.3.9",
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
