package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "sdl2_vk",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//rust/core/gfx/vk",
        "//third_party/cargo:sdl2",
        "//third_party/cargo:vk_sys",
        "//third_party/cargo/overrides/sdl2-sys:sdl2_sys",
    ],
)
