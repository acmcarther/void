package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cc_proto_compile")
load("//tools/bazel-ext/proto:rust.bzl", "rust_proto_library")

rust_library(
    name = "init",
    srcs = ["init.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:fern",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:zcfg",
        "//cargo/vendor:zcfg_flag_parser",
    ],
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
    ],
)

rust_binary(
    name = "launcher",
    srcs = ["launcher.rs"],
    deps = [
        ":init",
        ":server",
    ],
)
