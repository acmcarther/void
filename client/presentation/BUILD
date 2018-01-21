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
    data = [
        ":toy_vert_shader",
        ":toy_frag_shader",
    ],
    deps = [
        ":vulkan",
        "//cargo:sdl2",
        ":sdl2_vulkan_interop",
    ],
)

load("//tools:glslang.bzl", "compile_spirv_shader")

compile_spirv_shader(
    name = "toy_vert_shader",
    src = ":toy.vert",
)

compile_spirv_shader(
    name = "toy_frag_shader",
    src = ":toy.frag",
)
