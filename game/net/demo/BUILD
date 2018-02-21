package(default_visibility = ["//visibility:public"])

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
)

rust_proto_library(
    name = "demo_proto",
    protos = [
        "demo.proto",
    ],
    with_grpc = False,
)

rust_library(
    name = "demo_state",
    srcs = ["demo_state.rs"],
    deps = [
        "//interconnect:state_api",
        "//third_party/cargo:log",
        ":demo_proto",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_binary(
    name = "local_demo",
    srcs = ["local_demo.rs"],
    deps = [
        "//init",
        "//interconnect:state_acceptor",
        "//interconnect:state_acceptor_api",
        "//interconnect:state_api",
        "//interconnect:state_proto",
        "//interconnect:state_transmitter",
        "//interconnect:state_transmitter_api",
        "//third_party/cargo:log",
        ":demo_proto",
        ":demo_state",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_binary(
    name = "remote_demo",
    srcs = ["remote_demo.rs"],
    deps = [
        "//init",
        "//interconnect/netcode:netcode_api",
        "//interconnect/netcode:netcode_client",
        "//interconnect/netcode:netcode_global",
        "//interconnect/netcode:netcode_server",
        "//interconnect:state_acceptor",
        "//interconnect:state_acceptor_api",
        "//interconnect:state_api",
        "//interconnect:state_proto",
        "//interconnect:state_transmitter",
        "//interconnect:state_transmitter_api",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:lazy_static",
        ":demo_proto",
        ":demo_state",
        "@custom_rust_protobuf//:protobuf",
        "@netcode_io//:netcode_io_sys",
    ],
)
