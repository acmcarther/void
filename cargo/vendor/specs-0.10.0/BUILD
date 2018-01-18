"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//cargo:__subpackages__"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)
# Unsupported target "async" with type "example" omitted
# Unsupported target "basic" with type "example" omitted
# Unsupported target "bitset" with type "example" omitted
# Unsupported target "cluster_bomb" with type "example" omitted
# Unsupported target "common" with type "example" omitted
# Unsupported target "full" with type "example" omitted
# Unsupported target "parallel" with type "bench" omitted
# Unsupported target "serialize" with type "example" omitted

rust_library(
    name = "specs",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/crossbeam-0.3.0:crossbeam",
        "//cargo/vendor/derivative-1.0.0:derivative",
        "//cargo/vendor/fnv-1.0.6:fnv",
        "//cargo/vendor/hibitset-0.3.2:hibitset",
        "//cargo/vendor/mopa-0.2.2:mopa",
        "//cargo/vendor/rayon-0.8.2:rayon",
        "//cargo/vendor/shred-0.5.0:shred",
        "//cargo/vendor/shred-derive-0.3.0:shred_derive",
        "//cargo/vendor/tuple_utils-0.2.0:tuple_utils",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "storage" with type "bench" omitted
# Unsupported target "tests" with type "test" omitted
# Unsupported target "world" with type "bench" omitted
