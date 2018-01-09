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
    name = "glsl_to_spirv_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build/build.rs",
    data = glob([
      "*"
    ]),
    deps = [
      "//cargo/vendor/cmake-0.1.29:cmake",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    visibility = ["//visibility:private"],
)

genrule(
    name = "glsl_to_spirv_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["glsl_to_spirv_out_dir_outputs.tar.gz"],
    tools = [":glsl_to_spirv_build_script"],
    local = 1,
    cmd = "mkdir glsl_to_spirv_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/cargo/vendor/glsl-to-spirv-0.1.3\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        
        + " export OUT_DIR=$$PWD/glsl_to_spirv_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :glsl_to_spirv_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
rust_library(
    name = "glsl_to_spirv",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/tempdir-0.3.5:tempdir",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    out_dir_tar = ":glsl_to_spirv_build_script_executor",
    crate_features = [
        
    ],
)
# Unsupported target "test" with type "test" omitted
