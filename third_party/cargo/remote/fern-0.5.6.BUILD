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


# Unsupported target "channel_logging" with type "test" omitted
# Unsupported target "cmd-program" with type "example" omitted
# Unsupported target "colored" with type "example" omitted

rust_library(
    name = "fern",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__log__0_4_5//:log",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.5.6",
    crate_features = [
    ],
)

# Unsupported target "file_logging" with type "test" omitted
# Unsupported target "global_logging" with type "test" omitted
# Unsupported target "meta-logging" with type "example" omitted
# Unsupported target "meta_logging" with type "test" omitted
# Unsupported target "panic_logging" with type "test" omitted
# Unsupported target "pretty-colored" with type "example" omitted
# Unsupported target "support" with type "test" omitted
# Unsupported target "syslog" with type "example" omitted
# Unsupported target "syslog3" with type "example" omitted
# Unsupported target "write_logging" with type "test" omitted
