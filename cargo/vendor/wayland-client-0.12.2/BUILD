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
    name = "wayland_client_build_script",
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
      "cursor",
      "default",
      "dlopen",
      "egl",
      "wayland-sys",
    ],
    visibility = ["//visibility:private"],
)

genrule(
    name = "wayland_client_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["wayland_client_out_dir_outputs.tar.gz"],
    tools = [":wayland_client_build_script"],
    local = 1,
    cmd = "mkdir wayland_client_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/cargo/vendor/wayland-client-0.12.2\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export CARGO_FEATURE_CURSOR=1;"
        + " export CARGO_FEATURE_DEFAULT=1;"
        + " export CARGO_FEATURE_DLOPEN=1;"
        + " export CARGO_FEATURE_EGL=1;"
        + " export CARGO_FEATURE_WAYLAND_SYS=1;"
        + " export OUT_DIR=$$PWD/wayland_client_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :wayland_client_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
# Unsupported target "simple_client" with type "example" omitted
# Unsupported target "simple_window" with type "example" omitted
rust_library(
    name = "wayland_client",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-1.0.1:bitflags",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/token_store-0.1.2:token_store",
        "//cargo/vendor/wayland-sys-0.12.2:wayland_sys",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    out_dir_tar = ":wayland_client_build_script_executor",
    crate_features = [
        "cursor",
        "default",
        "dlopen",
        "egl",
        "wayland-sys",
    ],
)
