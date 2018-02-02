"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "alarm" with type "test" omitted
# Unsupported target "greeter_client" with type "example" omitted
# Unsupported target "greeter_server" with type "example" omitted

rust_library(
    name = "grpcio",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__futures__0_1_17//:futures",
        "@raze__grpcio_sys__0_2_0//:grpcio_sys",
        "@raze__libc__0_2_36//:libc",
        "@raze__log__0_3_9//:log",
        "@raze__protobuf__1_4_3//:protobuf",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "grpcio-sys",
        "protobuf",
        "protobuf-codec",
        "secure",
    ],
)

# Unsupported target "health_check" with type "test" omitted
# Unsupported target "route_guide_client" with type "example" omitted
# Unsupported target "route_guide_server" with type "example" omitted
