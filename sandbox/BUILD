package(default_visibility = ["//visibility:public"])

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "helloworld",
    protos = [
        "helloworld.proto",
    ],
    with_grpc = True)


load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
)

rust_binary(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        "//cargo:futures",
        "//cargo:grpcio",
        "//cargo:log",
        "//cargo:fern",
        "//cargo:chrono",
        ":helloworld",
    ],
)

rust_binary(
    name = "client",
    srcs = ["client.rs"],
    deps = [
        "//cargo:futures",
        "//cargo:grpcio",
        "//cargo:log",
        "//cargo:fern",
        "//cargo:chrono",
        ":helloworld",
    ],
)

