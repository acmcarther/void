package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "node",
    srcs = ["node.rs"],
    deps = [
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
        ":node",
        "//third_party/cargo:chrono",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_test(
    name = "stores_test",
    deps = [":stores"],
)

rust_library(
    name = "physics_service",
    srcs = ["physics_service.rs"],
    deps = [
        ":node",
        ":physics_proto",
        "//third_party/cargo:chrono",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
    ],
)

rust_test(
    name = "physics_service_test",
    deps = [":physics_service"],
)

load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_proto_library(
    name = "physics_proto",
    protos = [
        "physics.proto",
    ],
)
