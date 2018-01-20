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
rust_binary(
    name = "bindgen_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    data = glob([
      "*"
    ]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
      "default",
      "env_logger",
      "log",
      "logging",
    ],
    visibility = ["//visibility:private"],
)

genrule(
    name = "bindgen_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["bindgen_out_dir_outputs.tar.gz"],
    tools = [":bindgen_build_script"],
    local = 1,
    cmd = "mkdir bindgen_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_ENV_LOGGER=1;"
        + " export CARGO_FEATURE_LOG=1;"
        + " export CARGO_FEATURE_LOGGING=1;"
        + " export OUT_DIR=$$PWD/bindgen_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :bindgen_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_bindgen",
    crate_root = "src/main.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":bindgen",
        "@raze__cexpr__0_2_2//:cexpr",
        "@raze__cfg_if__0_1_2//:cfg_if",
        "@raze__clang_sys__0_21_1//:clang_sys",
        "@raze__clap__2_29_2//:clap",
        "@raze__env_logger__0_4_3//:env_logger",
        "@raze__lazy_static__1_0_0//:lazy_static",
        "@raze__log__0_3_9//:log",
        "@raze__peeking_take_while__0_1_2//:peeking_take_while",
        "@raze__quote__0_3_15//:quote",
        "@raze__regex__0_2_5//:regex",
        "@raze__which__1_0_3//:which",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":bindgen_build_script_executor",
    crate_features = [
        "default",
        "env_logger",
        "log",
        "logging",
    ],
)


rust_library(
    name = "bindgen",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__cexpr__0_2_2//:cexpr",
        "@raze__cfg_if__0_1_2//:cfg_if",
        "@raze__clang_sys__0_21_1//:clang_sys",
        "@raze__clap__2_29_2//:clap",
        "@raze__env_logger__0_4_3//:env_logger",
        "@raze__lazy_static__1_0_0//:lazy_static",
        "@raze__log__0_3_9//:log",
        "@raze__peeking_take_while__0_1_2//:peeking_take_while",
        "@raze__quote__0_3_15//:quote",
        "@raze__regex__0_2_5//:regex",
        "@raze__which__1_0_3//:which",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":bindgen_build_script_executor",
    crate_features = [
        "default",
        "env_logger",
        "log",
        "logging",
    ],
)

