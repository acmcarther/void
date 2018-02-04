package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
)

rust_binary(
    name = "client",
    srcs = ["client.rs"],
    deps = [
        "//init",
        "//interconnect",
        "//cargo:log",
        "//cargo:futures",
        "//cargo:grpcio",
        "//cargo:zcfg",
        "//cargo:lazy_static",
    ]
)
