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
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//init",
        ":physics",
        ":base_server",
        ":physics_proto",
        ":control_proto",
        ":stores",
    ]
)

rust_library(
    name = "base_server",
    srcs = ["base_server.rs"],
    deps = [
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:lazy_static",
    ]
)

rust_library(
    name = "physics",
    srcs = ["physics.rs"],
    deps = [
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:lazy_static",
        ":physics_proto",
        ":base_server",
        ":stores",
    ]
)


rust_library(
    name = "stores",
    srcs = ["stores.rs"],
    deps = [
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:lazy_static",
        ":physics_proto",
        ":control_proto",
        ":base_server",
    ]
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

