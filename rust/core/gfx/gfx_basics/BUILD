package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "gfx_basics",
    srcs = glob(["src/**/*.rs"]),
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//rust/core/math",
        "//third_party/cargo:sdl2",
    ],
)
