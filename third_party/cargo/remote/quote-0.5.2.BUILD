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



rust_library(
    name = "quote",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__proc_macro2__0_3_8//:proc_macro2",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.5.2",
    crate_features = [
        "default",
        "proc-macro",
        "proc-macro2",
    ],
)

# Unsupported target "test" with type "test" omitted
