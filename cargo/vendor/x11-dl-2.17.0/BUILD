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
    name = "x11_dl_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    data = glob([
      "*"
    ]),
    deps = [
      "//cargo/vendor/pkg-config-0.3.9:pkg_config",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    visibility = ["//visibility:private"],
)

genrule(
    name = "x11_dl_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["x11_dl_out_dir_outputs.tar.gz"],
    tools = [":x11_dl_build_script"],
    local = 1,
    cmd = "mkdir x11_dl_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/cargo/vendor/x11-dl-2.17.0\";"
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
        "//cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//cargo/vendor/libc-0.2.34:libc",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    out_dir_tar = ":x11_dl_build_script_executor",
    crate_features = [
        
    ],
)
