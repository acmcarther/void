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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "clones" with type "test" omitted
# Unsupported target "cpu_monitor" with type "example" omitted
# Unsupported target "debug" with type "test" omitted
# Unsupported target "intersperse" with type "test" omitted
# Unsupported target "producer_split_at" with type "test" omitted

rust_library(
    name = "rustc_rayon",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__either__1_5_0//:either",
        "@raze__rustc_rayon_core__0_1_1//:rustc_rayon_core",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.1.1",
    crate_features = [
    ],
)

# Unsupported target "sort-panic-safe" with type "test" omitted
