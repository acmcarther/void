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
  "unencumbered", # "CC0-1.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_binary(
    name = "x11_dl_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    deps = [
        "@raze__pkg_config__0_3_14//:pkg_config",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
    data = glob(["*"]),
    version = "2.18.3",
    visibility = ["//visibility:private"],
)

genrule(
    name = "x11_dl_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["x11_dl_out_dir_outputs.tar.gz"],
    tools = [
      ":x11_dl_build_script",
    ],
    local = 1,
    cmd = "mkdir -p x11_dl_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/x11_dl_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :x11_dl_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "hello-world" with type "example" omitted

rust_library(
    name = "x11_dl",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__lazy_static__1_1_0//:lazy_static",
        "@raze__libc__0_2_43//:libc",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":x11_dl_build_script_executor",
    version = "2.18.3",
    crate_features = [
    ],
)

