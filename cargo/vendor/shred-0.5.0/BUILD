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


# Unsupported target "basic_dispatch" with type "example" omitted
# Unsupported target "bench" with type "bench" omitted
# Unsupported target "custom_bundle" with type "example" omitted
# Unsupported target "derive_bundle" with type "example" omitted
# Unsupported target "dispatch" with type "test" omitted
# Unsupported target "fetch_opt" with type "example" omitted
# Unsupported target "generic_derive" with type "example" omitted
# Unsupported target "par_seq" with type "example" omitted
# Unsupported target "seq_dispatch" with type "example" omitted
rust_library(
    name = "shred",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/arrayvec-0.3.24:arrayvec",
        "//cargo/vendor/fnv-1.0.6:fnv",
        "//cargo/vendor/mopa-0.2.2:mopa",
        "//cargo/vendor/pulse-0.5.3:pulse",
        "//cargo/vendor/rayon-0.8.2:rayon",
        "//cargo/vendor/shred-derive-0.3.0:shred_derive",
        "//cargo/vendor/smallvec-0.4.4:smallvec",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
# Unsupported target "thread_local" with type "example" omitted
