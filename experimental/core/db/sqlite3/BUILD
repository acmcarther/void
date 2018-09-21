load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        "//core:init",
        "//third_party/cargo:log",
        "//third_party/cargo:rusqlite",
        "//third_party/cargo:time",
    ],
)
