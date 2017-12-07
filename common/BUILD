package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

rust_library(
    name = "monitoring",
    srcs = ["monitoring.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:futures",
        "//cargo/vendor:hyper",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:radix_trie",
        "//cargo/vendor:zcfg",
    ],
)

rust_test(
    name = "monitoring_tests",
    srcs = ["monitoring_tests.rs"],
    deps = [
        ":monitoring",
        "//cargo/vendor:chrono",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:zcfg",
    ],
)

rust_library(
    name = "init",
    srcs = ["init.rs"],
    deps = [
        ":monitoring",
        "//cargo/vendor:chrono",
        "//cargo/vendor:fern",
        "//cargo/vendor:lazy_static",
        "//cargo/vendor:log",
        "//cargo/vendor:zcfg",
        "//cargo/vendor:zcfg_flag_parser",
    ],
)
