package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "sdl2_ext",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//third_party/cargo:sdl2",
    ],
)
