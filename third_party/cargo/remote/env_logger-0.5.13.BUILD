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


# Unsupported target "custom_default_format" with type "example" omitted
# Unsupported target "custom_format" with type "example" omitted
# Unsupported target "custom_logger" with type "example" omitted
# Unsupported target "default" with type "example" omitted
# Unsupported target "direct_logger" with type "example" omitted

rust_library(
    name = "env_logger",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__atty__0_2_11//:atty",
        "@raze__humantime__1_1_1//:humantime",
        "@raze__log__0_4_5//:log",
        "@raze__regex__1_0_5//:regex",
        "@raze__termcolor__1_0_4//:termcolor",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.5.13",
    crate_features = [
        "default",
        "regex",
    ],
)

# Unsupported target "log-in-log" with type "test" omitted
# Unsupported target "regexp_filter" with type "test" omitted
