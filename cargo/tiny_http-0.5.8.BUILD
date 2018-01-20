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

# Unsupported target "bench" with type "bench" omitted
# Unsupported target "hello-world" with type "example" omitted
# Unsupported target "input-tests" with type "test" omitted
# Unsupported target "network" with type "test" omitted
# Unsupported target "php-cgi" with type "example" omitted
# Unsupported target "readme-example" with type "example" omitted
# Unsupported target "serve-root" with type "example" omitted
# Unsupported target "simple-test" with type "test" omitted
# Unsupported target "ssl" with type "example" omitted

rust_library(
    name = "tiny_http",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__ascii__0_7_1//:ascii",
        "@raze__chrono__0_2_25//:chrono",
        "@raze__chunked_transfer__0_3_1//:chunked_transfer",
        "@raze__encoding__0_2_33//:encoding",
        "@raze__log__0_3_9//:log",
        "@raze__rustc_serialize__0_3_24//:rustc_serialize",
        "@raze__url__0_2_38//:url",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
    ],
)

# Unsupported target "websockets" with type "example" omitted
