package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
)

rust_binary(
    name = "client",
    srcs = ["client.rs"],
    deps = [
        "//core:init",
        "//game:interconnect",
        "//third_party/cargo:log",
        "//third_party/cargo:futures",
        "//third_party/cargo:grpcio",
        "//third_party/cargo:zcfg",
        "//third_party/cargo:lazy_static",
    ]
)
