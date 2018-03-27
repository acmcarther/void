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
        ":demo_proto",
        "//game/state/relay:state_api",
        "//third_party/cargo:log",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_binary(
    name = "local_demo",
    srcs = ["local_demo.rs"],
    deps = [
        ":demo_proto",
        ":demo_state",
        "//core:init",
        "//game/state/relay:state_acceptor",
        "//game/state/relay:state_acceptor_api",
        "//game/state/relay:state_api",
        "//game/state/relay:state_proto",
        "//game/state/relay:state_transmitter",
        "//game/state/relay:state_transmitter_api",
        "//third_party/cargo:log",
        "@custom_rust_protobuf//:protobuf",
    ],
)

rust_binary(
    name = "remote_demo",
    srcs = ["remote_demo.rs"],
    deps = [
        ":demo_proto",
        ":demo_state",
        "//core:init",
        "//core/net/low:netcode_api",
        "//core/net/low:netcode_client",
        "//core/net/low:netcode_global",
        "//core/net/low:netcode_server",
        "//game/state/relay:state_acceptor",
        "//game/state/relay:state_acceptor_api",
        "//game/state/relay:state_api",
        "//game/state/relay:state_proto",
        "//game/state/relay:state_transmitter",
        "//game/state/relay:state_transmitter_api",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "@custom_rust_protobuf//:protobuf",
        "@netcode_io//:netcode_io_sys",
    ],
)
