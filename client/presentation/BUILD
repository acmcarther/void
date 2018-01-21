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
    name = "sdl2_vulkan_interop",
    srcs = ["sdl2_vulkan_interop.rs"],
    deps = [
        ":vulkan",
        "//third_party/SDL2:sdl2_sys",
        "//cargo:sdl2",
        "//cargo:vk_sys",
    ],
)


rust_binary(
    name = "toy",
    srcs = ["toy.rs"],
    deps = [
        ":vulkan",
        "//cargo:sdl2",
        ":sdl2_vulkan_interop",
    ],
)
