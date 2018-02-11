package(default_visibility = ["//visibility:public"])

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "interconnect_proto",
    protos = [
        "interconnect.proto",
    ],
    with_grpc = True
)

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "client",
    srcs = ["client.rs"],
    deps = [
        "//cargo:lazy_static",
        "//cargo:log",
        "//init:init",
        "//cargo:zcfg",
        "//cargo:zcfg_flag_parser",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        "//cargo:lazy_static",
        "//cargo:log",
        "//init:init",
        "//cargo:zcfg",
        "//cargo:zcfg_flag_parser",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

rust_library(
    name = "netcode_api",
    srcs = ["netcode_api.rs"],
    deps = [
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)

