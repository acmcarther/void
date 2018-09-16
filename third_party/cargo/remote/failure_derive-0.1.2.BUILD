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
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "backtrace" with type "test" omitted
# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "custom_type_bounds" with type "test" omitted

rust_library(
    name = "failure_derive",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__proc_macro2__0_4_19//:proc_macro2",
        "@raze__quote__0_6_8//:quote",
        "@raze__syn__0_14_9//:syn",
        "@raze__synstructure__0_9_0//:synstructure",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.1.2",
    crate_features = [
    ],
)

# Unsupported target "no_derive_display" with type "test" omitted
# Unsupported target "tests" with type "test" omitted
# Unsupported target "wraps" with type "test" omitted
