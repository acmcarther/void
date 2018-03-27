package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
)

rust_binary(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        ":control_proto",
        ":physics",
        ":physics_proto",
        ":stores",
        "//core:init",
        "//game/node",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_library(
    name = "physics",
    srcs = ["physics.rs"],
    deps = [
        ":base_server",
        ":physics_proto",
        ":stores",
        "//third_party/cargo:chrono",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_library(
    name = "stores",
    srcs = ["stores.rs"],
    deps = [
        ":base_server",
        ":control_proto",
        ":physics_proto",
        "//third_party/cargo:chrono",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "physics_proto",
    protos = [
        "physics.proto",
    ],
)

rust_proto_library(
    name = "control_proto",
    protos = [
        "control.proto",
    ],
)
