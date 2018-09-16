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


# Unsupported target "async" with type "example" omitted
# Unsupported target "basic_dispatch" with type "example" omitted
# Unsupported target "bench" with type "bench" omitted
# Unsupported target "custom_bundle" with type "example" omitted
# Unsupported target "derive_bundle" with type "example" omitted
# Unsupported target "dispatch" with type "test" omitted
# Unsupported target "dyn_sys_data" with type "example" omitted
# Unsupported target "fetch_opt" with type "example" omitted
# Unsupported target "generic_derive" with type "example" omitted
# Unsupported target "nightly" with type "test" omitted
# Unsupported target "par_seq" with type "example" omitted
# Unsupported target "seq_dispatch" with type "example" omitted

rust_library(
    name = "shred",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__arrayvec__0_4_7//:arrayvec",
        "@raze__fxhash__0_2_1//:fxhash",
        "@raze__mopa__0_2_2//:mopa",
        "@raze__rayon__1_0_2//:rayon",
        "@raze__smallvec__0_6_5//:smallvec",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.7.0",
    crate_features = [
        "default",
        "parallel",
        "rayon",
    ],
)

# Unsupported target "thread_local" with type "example" omitted
