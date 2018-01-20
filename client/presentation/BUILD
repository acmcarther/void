package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "vulkan",
    srcs = ["vulkan.rs"],
    deps = [
        "//cargo:dylib",
        "//cargo:vk_sys",
    ],
)

rust_library(
    name = "sdl2_window",
    srcs = ["sdl2_window.rs"],
    deps = [
        "//third_party/SDL2:sdl2_sys",
        "//cargo:sdl2",
        "//cargo:x11_dl",
    ],
)


rust_binary(
    name = "presentation_toy",
    srcs = ["presentation_toy.rs"],
    deps = [
        ":vulkan",
        ":sdl2_window",
    ],
)

alias(name = "toy", actual = ":presentation_toy")
