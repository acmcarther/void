package(default_visibility = ["//visibility:public"])

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "interconnect_proto",
    protos = [
        "interconnect.proto",
    ],
    with_grpc = True
)

rust_proto_library(
    name = "state_proto",
    protos = [
        "state.proto",
    ],
    with_grpc = False
)


load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_library(
    name = "state",
    srcs = ["state.rs"],
    deps = [
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
    ]
)

rust_library(
    name = "state_acceptor_api",
    srcs = ["state_acceptor_api.rs"],
    deps = [
        ":state",
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
    ]
)

rust_library(
    name = "state_relay_api",
    srcs = ["state_relay_api.rs"],
    deps = [
        ":state",
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
    ]
)

rust_library(
    name = "state_relay",
    srcs = ["state_relay.rs"],
    deps = [
        ":state",
        ":state_relay_api",
        ":state_proto",
        "@custom_rust_protobuf//:protobuf",
        "//cargo:log",
    ]
)
