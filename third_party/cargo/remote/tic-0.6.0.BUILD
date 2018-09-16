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


# Unsupported target "allan" with type "example" omitted
# Unsupported target "benchmark" with type "example" omitted

rust_library(
    name = "tic",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__allan__0_2_4//:allan",
        "@raze__clocksource__0_4_0//:clocksource",
        "@raze__fnv__1_0_6//:fnv",
        "@raze__getopts__0_2_18//:getopts",
        "@raze__heatmap__0_6_6//:heatmap",
        "@raze__histogram__0_6_9//:histogram",
        "@raze__log__0_3_9//:log",
        "@raze__mio__0_6_16//:mio",
        "@raze__mio_extras__2_0_5//:mio_extras",
        "@raze__mpmc__0_1_5//:mpmc",
        "@raze__time__0_1_40//:time",
        "@raze__tiny_http__0_5_9//:tiny_http",
        "@raze__waterfall__0_7_1//:waterfall",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.6.0",
    crate_features = [
        "default",
    ],
)

