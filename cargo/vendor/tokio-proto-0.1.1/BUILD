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


# Unsupported target "simple_client_proto" with type "test" omitted
# Unsupported target "simple_framed" with type "test" omitted
# Unsupported target "test_multiplex_client" with type "test" omitted
# Unsupported target "test_multiplex_deadlock" with type "test" omitted
# Unsupported target "test_multiplex_server" with type "test" omitted
# Unsupported target "test_pipeline_client" with type "test" omitted
# Unsupported target "test_pipeline_server" with type "test" omitted
rust_library(
    name = "tokio_proto",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/futures-0.1.17:futures",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/net2-0.2.31:net2",
        "//cargo/vendor/rand-0.3.18:rand",
        "//cargo/vendor/slab-0.3.0:slab",
        "//cargo/vendor/smallvec-0.2.1:smallvec",
        "//cargo/vendor/take-0.1.0:take",
        "//cargo/vendor/tokio-core-0.1.10:tokio_core",
        "//cargo/vendor/tokio-io-0.1.4:tokio_io",
        "//cargo/vendor/tokio-service-0.1.0:tokio_service",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
