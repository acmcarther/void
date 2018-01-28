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
    name = "cgmath_build_script",
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
    ],
    visibility = ["//visibility:private"],
)

genrule(
    name = "cgmath_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["cgmath_out_dir_outputs.tar.gz"],
    tools = [":cgmath_build_script"],
    local = 1,
    cmd = "mkdir cgmath_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/cgmath_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :cgmath_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)

# Unsupported target "angle" with type "test" omitted

rust_library(
    name = "cgmath",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__approx__0_1_1//:approx",
        "@raze__num_traits__0_1_41//:num_traits",
        "@raze__rand__0_4_2//:rand",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":cgmath_build_script_executor",
    crate_features = [
    ],
)

# Unsupported target "construction" with type "bench" omitted
# Unsupported target "mat" with type "bench" omitted
# Unsupported target "matrix" with type "test" omitted
# Unsupported target "point" with type "test" omitted
# Unsupported target "projection" with type "test" omitted
# Unsupported target "quat" with type "bench" omitted
# Unsupported target "quaternion" with type "test" omitted
# Unsupported target "rotation" with type "test" omitted
# Unsupported target "swizzle" with type "test" omitted
# Unsupported target "transform" with type "test" omitted
# Unsupported target "vec" with type "bench" omitted
# Unsupported target "vector" with type "test" omitted
# Unsupported target "vector4f32" with type "test" omitted
