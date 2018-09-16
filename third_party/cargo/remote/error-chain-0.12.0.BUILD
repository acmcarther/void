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


# Unsupported target "all" with type "example" omitted
# Unsupported target "chain_err" with type "example" omitted
# Unsupported target "doc" with type "example" omitted

rust_library(
    name = "error_chain",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__backtrace__0_3_9//:backtrace",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.12.0",
    crate_features = [
        "backtrace",
    ],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_has_backtrace",
    crate_root = "src/bin/has_backtrace.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":error_chain",
        "@raze__backtrace__0_3_9//:backtrace",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.12.0",
    crate_features = [
        "backtrace",
    ],
)

# Unsupported target "quick_main" with type "test" omitted
# Unsupported target "quickstart" with type "example" omitted
# Unsupported target "size" with type "example" omitted
# Unsupported target "tests" with type "test" omitted
