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
# Unsupported target "basic" with type "example" omitted
# Unsupported target "benches_main" with type "bench" omitted
# Unsupported target "bitset" with type "example" omitted
# Unsupported target "cluster_bomb" with type "example" omitted
# Unsupported target "common" with type "example" omitted
# Unsupported target "full" with type "example" omitted
# Unsupported target "parallel" with type "bench" omitted
# Unsupported target "saveload" with type "example" omitted

rust_library(
    name = "specs",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__crossbeam__0_3_2//:crossbeam",
        "@raze__derivative__1_0_0//:derivative",
        "@raze__fnv__1_0_6//:fnv",
        "@raze__hibitset__0_5_2//:hibitset",
        "@raze__log__0_4_5//:log",
        "@raze__mopa__0_2_2//:mopa",
        "@raze__rayon__1_0_2//:rayon",
        "@raze__shred__0_7_0//:shred",
        "@raze__shred_derive__0_5_0//:shred_derive",
        "@raze__shrev__1_0_1//:shrev",
        "@raze__tuple_utils__0_2_0//:tuple_utils",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.12.2",
    crate_features = [
    ],
)

# Unsupported target "tests" with type "test" omitted
# Unsupported target "track" with type "example" omitted
# Unsupported target "world" with type "bench" omitted
