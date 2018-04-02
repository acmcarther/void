package(default_visibility = ["//visibility:public"])

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "state_proto",
    protos = [
        "state.proto",
    ],
    with_grpc = False,
)

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_library(
    name = "state_api",
    srcs = ["state_api.rs"],
    deps = [
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_library(
    name = "state_acceptor",
    srcs = ["state_acceptor.rs"],
    deps = [
        ":state_acceptor_api",
        ":state_api",
        ":state_proto",
        "//third_party/cargo:log",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_library(
    name = "state_acceptor_api",
    srcs = ["state_acceptor_api.rs"],
    deps = [
        ":state_api",
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_library(
    name = "state_transmitter_api",
    srcs = ["state_transmitter_api.rs"],
    deps = [
        ":state_api",
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_library(
    name = "state_transmitter",
    srcs = ["state_transmitter.rs"],
    deps = [
        ":state_api",
        ":state_proto",
        ":state_transmitter_api",
        "//third_party/cargo:log",
        "@custom_rust_protobuf//:protobuf",
    ],
)
