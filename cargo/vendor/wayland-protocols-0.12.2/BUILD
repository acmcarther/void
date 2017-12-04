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

rust_binary(
    name = "wayland_protocols_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    data = glob([
      "*"
    ]),
    deps = [
      "//cargo/vendor/wayland-scanner-0.12.2:wayland_scanner",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
      "client",
      "unstable_protocols",
      "wayland-client",
    ],
    visibility = ["//visibility:private"],
)

genrule(
    name = "wayland_protocols_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["wayland_protocols_out_dir_outputs.tar.gz"],
    tools = [":wayland_protocols_build_script"],
    local = 1,
    cmd = "mkdir wayland_protocols_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/cargo/vendor/wayland-protocols-0.12.2\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_CLIENT=1;"
        + " export CARGO_FEATURE_UNSTABLE_PROTOCOLS=1;"
        + " export CARGO_FEATURE_WAYLAND_CLIENT=1;"
        + " export OUT_DIR=$$PWD/wayland_protocols_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :wayland_protocols_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
rust_library(
    name = "wayland_protocols",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-1.0.1:bitflags",
        "//cargo/vendor/wayland-client-0.12.2:wayland_client",
        "//cargo/vendor/wayland-sys-0.12.2:wayland_sys",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    out_dir_tar = ":wayland_protocols_build_script_executor",
    crate_features = [
        "client",
        "unstable_protocols",
        "wayland-client",
    ],
)
