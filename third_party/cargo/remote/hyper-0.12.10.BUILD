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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "client" with type "example" omitted
# Unsupported target "client" with type "test" omitted
# Unsupported target "client_json" with type "example" omitted
# Unsupported target "echo" with type "example" omitted
# Unsupported target "end_to_end" with type "bench" omitted
# Unsupported target "hello" with type "example" omitted

rust_library(
    name = "hyper",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bytes__0_4_10//:bytes",
        "@raze__futures__0_1_24//:futures",
        "@raze__futures_cpupool__0_1_8//:futures_cpupool",
        "@raze__h2__0_1_12//:h2",
        "@raze__http__0_1_13//:http",
        "@raze__httparse__1_3_2//:httparse",
        "@raze__iovec__0_1_2//:iovec",
        "@raze__itoa__0_4_3//:itoa",
        "@raze__log__0_4_5//:log",
        "@raze__net2__0_2_33//:net2",
        "@raze__time__0_1_40//:time",
        "@raze__tokio__0_1_8//:tokio",
        "@raze__tokio_executor__0_1_4//:tokio_executor",
        "@raze__tokio_io__0_1_8//:tokio_io",
        "@raze__tokio_reactor__0_1_5//:tokio_reactor",
        "@raze__tokio_tcp__0_1_1//:tokio_tcp",
        "@raze__tokio_timer__0_2_6//:tokio_timer",
        "@raze__want__0_0_6//:want",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.12.10",
    crate_features = [
        "__internal_flaky_tests",
        "default",
        "futures-cpupool",
        "net2",
        "runtime",
        "tokio",
        "tokio-executor",
        "tokio-reactor",
        "tokio-tcp",
        "tokio-timer",
    ],
)

# Unsupported target "integration" with type "test" omitted
# Unsupported target "multi_server" with type "example" omitted
# Unsupported target "params" with type "example" omitted
# Unsupported target "send_file" with type "example" omitted
# Unsupported target "server" with type "bench" omitted
# Unsupported target "server" with type "test" omitted
# Unsupported target "state" with type "example" omitted
# Unsupported target "upgrades" with type "example" omitted
# Unsupported target "web_api" with type "example" omitted
