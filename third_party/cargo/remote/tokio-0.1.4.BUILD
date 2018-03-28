"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "buffered" with type "test" omitted
# Unsupported target "chat" with type "example" omitted
# Unsupported target "chat-combinator" with type "example" omitted
# Unsupported target "connect" with type "example" omitted
# Unsupported target "current_thread" with type "test" omitted
# Unsupported target "drop-core" with type "test" omitted
# Unsupported target "echo" with type "example" omitted
# Unsupported target "echo-udp" with type "example" omitted
# Unsupported target "echo2" with type "test" omitted
# Unsupported target "global" with type "test" omitted
# Unsupported target "global2" with type "test" omitted
# Unsupported target "hello_world" with type "example" omitted
# Unsupported target "latency" with type "bench" omitted
# Unsupported target "line-frames" with type "test" omitted
# Unsupported target "mio-ops" with type "bench" omitted
# Unsupported target "pipe-hup" with type "test" omitted
# Unsupported target "proxy" with type "example" omitted
# Unsupported target "runtime" with type "test" omitted
# Unsupported target "tcp" with type "bench" omitted
# Unsupported target "tcp2" with type "test" omitted
# Unsupported target "tinydb" with type "example" omitted
# Unsupported target "tinyhttp" with type "example" omitted

rust_library(
    name = "tokio",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__futures__0_1_20//:futures",
        "@raze__mio__0_6_14//:mio",
        "@raze__tokio_executor__0_1_1//:tokio_executor",
        "@raze__tokio_io__0_1_6//:tokio_io",
        "@raze__tokio_reactor__0_1_1//:tokio_reactor",
        "@raze__tokio_tcp__0_1_0//:tokio_tcp",
        "@raze__tokio_threadpool__0_1_1//:tokio_threadpool",
        "@raze__tokio_udp__0_1_0//:tokio_udp",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
    ],
)

# Unsupported target "udp-client" with type "example" omitted
# Unsupported target "udp-codec" with type "example" omitted
