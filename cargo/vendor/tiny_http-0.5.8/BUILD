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
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/ascii-0.7.1:ascii",
        "//cargo/vendor/chrono-0.2.25:chrono",
        "//cargo/vendor/chunked_transfer-0.3.1:chunked_transfer",
        "//cargo/vendor/encoding-0.2.33:encoding",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/rustc-serialize-0.3.24:rustc_serialize",
        "//cargo/vendor/url-0.2.38:url",
        
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
