package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "procedural",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//rust/core/gfx/gfx_basics",
        "//rust/core/math",
        "//third_party/cargo:itertools",
    ],
)

rust_test(
    name = "procedural_test",
    deps = [":procedural"],
)
