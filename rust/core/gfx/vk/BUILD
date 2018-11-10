package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "vk",
    srcs = glob(["src/**/*.rs"]),
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//rust/core/gfx/gfx_basics",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:dylib",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:vk_sys",
    ],
)

rust_binary(
    name = "example_triangle",
    srcs = ["examples/triangle.rs"],
    deps = [
        ":vk",
        "//rust/core/base",
        "//rust/core/gfx/vk/sdl2_vk",
        "//rust/core/net/web/spect/spect_easy",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:sdl2",
        "//third_party/cargo:zcfg",
    ],
)
