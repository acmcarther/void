"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "buffered" with type "test" omitted
# Unsupported target "chain" with type "test" omitted
# Unsupported target "chat" with type "example" omitted
# Unsupported target "compress" with type "example" omitted
# Unsupported target "connect" with type "example" omitted
# Unsupported target "echo" with type "example" omitted
# Unsupported target "echo" with type "test" omitted
# Unsupported target "echo-threads" with type "example" omitted
# Unsupported target "echo-udp" with type "example" omitted
# Unsupported target "hello" with type "example" omitted
# Unsupported target "interval" with type "test" omitted
# Unsupported target "latency" with type "bench" omitted
# Unsupported target "limit" with type "test" omitted
# Unsupported target "line-frames" with type "test" omitted
# Unsupported target "mio-ops" with type "bench" omitted
# Unsupported target "pipe-hup" with type "test" omitted
# Unsupported target "proxy" with type "example" omitted
# Unsupported target "sink" with type "example" omitted
# Unsupported target "spawn" with type "test" omitted
# Unsupported target "stream-buffered" with type "test" omitted
# Unsupported target "tcp" with type "bench" omitted
# Unsupported target "tcp" with type "test" omitted
# Unsupported target "timeout" with type "test" omitted
# Unsupported target "tinydb" with type "example" omitted
# Unsupported target "tinyhttp" with type "example" omitted

rust_library(
    name = "tokio_core",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bytes__0_4_6//:bytes",
        "@raze__futures__0_1_17//:futures",
        "@raze__iovec__0_1_1//:iovec",
        "@raze__log__0_4_1//:log",
        "@raze__mio__0_6_12//:mio",
        "@raze__scoped_tls__0_1_0//:scoped_tls",
        "@raze__slab__0_4_0//:slab",
        "@raze__tokio_io__0_1_4//:tokio_io",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "udp" with type "test" omitted
# Unsupported target "udp-codec" with type "example" omitted
