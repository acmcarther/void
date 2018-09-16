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


# Unsupported target "config_log" with type "test" omitted
# Unsupported target "deny_single_threaded_sqlite_config" with type "test" omitted
# Unsupported target "lib" with type "bench" omitted

rust_library(
    name = "rusqlite",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__1_0_4//:bitflags",
        "@raze__lru_cache__0_1_1//:lru_cache",
        "@raze__time__0_1_40//:time",
        "@//third_party/cargo/overrides/libsqlite3-sys:libsqlite3_sys",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.14.0",
    crate_features = [
    ],
)

# Unsupported target "vtab" with type "test" omitted
