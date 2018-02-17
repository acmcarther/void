package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")


rust_library(
    name = "grpcio_sys",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__libc__0_2_36//:libc",
        "@com_github_grpc_grpc//:grpc",
        ":grpc_wrap",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "secure",
    ],
)

cc_library(
    name = "grpc_wrap",
    srcs = ["grpc_wrap.c"],
    deps = [
        "@com_github_grpc_grpc//:grpc",
    ]
)
