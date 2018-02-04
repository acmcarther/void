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
    name = "grpcio_proto_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    data = glob([
      "*"
    ]),
    deps = [
        "@custom_rust_protobuf//:protobuf",
        "@custom_rust_grpc_compiler//:grpcio_compiler",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        "-C opt-level=2",
    ],
    crate_features = [
    ],
    visibility = ["//visibility:private"],
)

genrule(
    name = "grpcio_proto_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["grpcio_proto_out_dir_outputs.tar.gz"],
    tools = [":grpcio_proto_build_script"],
    local = 1,
    cmd = "mkdir grpcio_proto_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/$$(dirname $(location :Cargo.toml))\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/grpcio_proto_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :grpcio_proto_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)


rust_library(
    name = "grpcio_proto",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__futures__0_1_17//:futures",
        "@raze__grpcio__0_2_0//:grpcio",
        "@custom_rust_protobuf//:protobuf",
        "@custom_rust_grpc_compiler//:grpcio_compiler",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "-C opt-level=2",
        "--target=x86_64-unknown-linux-gnu",
    ],
    out_dir_tar = ":grpcio_proto_build_script_executor",
    crate_features = [
    ],
)

