package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_binary(
    name = "nio_client",
    srcs = ["nio_client.rs"],
    deps = [
        "//init",
        "//cargo:log",
        "//cargo:lazy_static",
        "//cargo:zcfg",
        "//cargo:libc",
        "@netcode_io//:netcode_io_sys"
    ],
)


