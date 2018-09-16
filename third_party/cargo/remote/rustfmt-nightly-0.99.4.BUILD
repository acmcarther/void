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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_binary(
    name = "rustfmt_nightly_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    deps = [
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
      "cargo-fmt",
      "default",
      "rustfmt-format-diff",
    ],
    data = glob(["*"]),
    version = "0.99.4",
    visibility = ["//visibility:private"],
)

genrule(
    name = "rustfmt_nightly_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["rustfmt_nightly_out_dir_outputs.tar.gz"],
    tools = [
      ":rustfmt_nightly_build_script",
    ],
    local = 1,
    cmd = "mkdir -p rustfmt_nightly_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_CARGO_FMT=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_RUSTFMT_FORMAT_DIFF=1;"
        + " export OUT_DIR=$$PWD/rustfmt_nightly_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :rustfmt_nightly_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_cargo_fmt",
    crate_root = "src/cargo-fmt/main.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":rustfmt_nightly",
        "@raze__cargo_metadata__0_6_0//:cargo_metadata",
        "@raze__derive_new__0_5_5//:derive_new",
        "@raze__diff__0_1_11//:diff",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__failure__0_1_2//:failure",
        "@raze__getopts__0_2_18//:getopts",
        "@raze__isatty__0_1_9//:isatty",
        "@raze__itertools__0_7_8//:itertools",
        "@raze__log__0_4_5//:log",
        "@raze__regex__1_0_5//:regex",
        "@raze__rustc_ap_rustc_target__237_0_0//:rustc_ap_rustc_target",
        "@raze__rustc_ap_syntax__237_0_0//:rustc_ap_syntax",
        "@raze__rustc_ap_syntax_pos__237_0_0//:rustc_ap_syntax_pos",
        "@raze__serde__1_0_79//:serde",
        "@raze__serde_derive__1_0_79//:serde_derive",
        "@raze__serde_json__1_0_27//:serde_json",
        "@raze__term__0_5_1//:term",
        "@raze__toml__0_4_6//:toml",
        "@raze__unicode_segmentation__1_2_1//:unicode_segmentation",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":rustfmt_nightly_build_script_executor",
    version = "0.99.4",
    crate_features = [
        "cargo-fmt",
        "default",
        "rustfmt-format-diff",
    ],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_git_rustfmt",
    crate_root = "src/git-rustfmt/main.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":rustfmt_nightly",
        "@raze__cargo_metadata__0_6_0//:cargo_metadata",
        "@raze__derive_new__0_5_5//:derive_new",
        "@raze__diff__0_1_11//:diff",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__failure__0_1_2//:failure",
        "@raze__getopts__0_2_18//:getopts",
        "@raze__isatty__0_1_9//:isatty",
        "@raze__itertools__0_7_8//:itertools",
        "@raze__log__0_4_5//:log",
        "@raze__regex__1_0_5//:regex",
        "@raze__rustc_ap_rustc_target__237_0_0//:rustc_ap_rustc_target",
        "@raze__rustc_ap_syntax__237_0_0//:rustc_ap_syntax",
        "@raze__rustc_ap_syntax_pos__237_0_0//:rustc_ap_syntax_pos",
        "@raze__serde__1_0_79//:serde",
        "@raze__serde_derive__1_0_79//:serde_derive",
        "@raze__serde_json__1_0_27//:serde_json",
        "@raze__term__0_5_1//:term",
        "@raze__toml__0_4_6//:toml",
        "@raze__unicode_segmentation__1_2_1//:unicode_segmentation",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":rustfmt_nightly_build_script_executor",
    version = "0.99.4",
    crate_features = [
        "cargo-fmt",
        "default",
        "rustfmt-format-diff",
    ],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_rustfmt",
    crate_root = "src/bin/main.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":rustfmt_nightly",
        "@raze__cargo_metadata__0_6_0//:cargo_metadata",
        "@raze__derive_new__0_5_5//:derive_new",
        "@raze__diff__0_1_11//:diff",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__failure__0_1_2//:failure",
        "@raze__getopts__0_2_18//:getopts",
        "@raze__isatty__0_1_9//:isatty",
        "@raze__itertools__0_7_8//:itertools",
        "@raze__log__0_4_5//:log",
        "@raze__regex__1_0_5//:regex",
        "@raze__rustc_ap_rustc_target__237_0_0//:rustc_ap_rustc_target",
        "@raze__rustc_ap_syntax__237_0_0//:rustc_ap_syntax",
        "@raze__rustc_ap_syntax_pos__237_0_0//:rustc_ap_syntax_pos",
        "@raze__serde__1_0_79//:serde",
        "@raze__serde_derive__1_0_79//:serde_derive",
        "@raze__serde_json__1_0_27//:serde_json",
        "@raze__term__0_5_1//:term",
        "@raze__toml__0_4_6//:toml",
        "@raze__unicode_segmentation__1_2_1//:unicode_segmentation",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":rustfmt_nightly_build_script_executor",
    version = "0.99.4",
    crate_features = [
        "cargo-fmt",
        "default",
        "rustfmt-format-diff",
    ],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_rustfmt_format_diff",
    crate_root = "src/format-diff/main.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":rustfmt_nightly",
        "@raze__cargo_metadata__0_6_0//:cargo_metadata",
        "@raze__derive_new__0_5_5//:derive_new",
        "@raze__diff__0_1_11//:diff",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__failure__0_1_2//:failure",
        "@raze__getopts__0_2_18//:getopts",
        "@raze__isatty__0_1_9//:isatty",
        "@raze__itertools__0_7_8//:itertools",
        "@raze__log__0_4_5//:log",
        "@raze__regex__1_0_5//:regex",
        "@raze__rustc_ap_rustc_target__237_0_0//:rustc_ap_rustc_target",
        "@raze__rustc_ap_syntax__237_0_0//:rustc_ap_syntax",
        "@raze__rustc_ap_syntax_pos__237_0_0//:rustc_ap_syntax_pos",
        "@raze__serde__1_0_79//:serde",
        "@raze__serde_derive__1_0_79//:serde_derive",
        "@raze__serde_json__1_0_27//:serde_json",
        "@raze__term__0_5_1//:term",
        "@raze__toml__0_4_6//:toml",
        "@raze__unicode_segmentation__1_2_1//:unicode_segmentation",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":rustfmt_nightly_build_script_executor",
    version = "0.99.4",
    crate_features = [
        "cargo-fmt",
        "default",
        "rustfmt-format-diff",
    ],
)


rust_library(
    name = "rustfmt_nightly",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__cargo_metadata__0_6_0//:cargo_metadata",
        "@raze__derive_new__0_5_5//:derive_new",
        "@raze__diff__0_1_11//:diff",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__failure__0_1_2//:failure",
        "@raze__getopts__0_2_18//:getopts",
        "@raze__isatty__0_1_9//:isatty",
        "@raze__itertools__0_7_8//:itertools",
        "@raze__log__0_4_5//:log",
        "@raze__regex__1_0_5//:regex",
        "@raze__rustc_ap_rustc_target__237_0_0//:rustc_ap_rustc_target",
        "@raze__rustc_ap_syntax__237_0_0//:rustc_ap_syntax",
        "@raze__rustc_ap_syntax_pos__237_0_0//:rustc_ap_syntax_pos",
        "@raze__serde__1_0_79//:serde",
        "@raze__serde_derive__1_0_79//:serde_derive",
        "@raze__serde_json__1_0_27//:serde_json",
        "@raze__term__0_5_1//:term",
        "@raze__toml__0_4_6//:toml",
        "@raze__unicode_segmentation__1_2_1//:unicode_segmentation",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":rustfmt_nightly_build_script_executor",
    version = "0.99.4",
    crate_features = [
        "cargo-fmt",
        "default",
        "rustfmt-format-diff",
    ],
)

