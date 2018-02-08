package(default_visibility = ["//visibility:public"])

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "interconnect",
    protos = [
        "interconnect.proto",
    ],
    with_grpc = False,
)

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "store",
    srcs = ["store.rs"],
    deps = [
        ":interconnect",
        "//cargo:futures",
        "//cargo:lazy_static",
        "//cargo:log",
        "//cargo:zcfg",
        "//init",
    ],
)
