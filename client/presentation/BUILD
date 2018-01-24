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
        "//cargo:sdl2",
        "//cargo:vk_sys",
        "//cargo/overrides/sdl2-sys:sdl2_sys",
    ],
)

rust_binary(
    name = "toy",
    srcs = ["toy.rs"],
    data = [
        ":toy_frag_shader",
        ":toy_vert_shader",
    ],
    deps = [
        ":sdl2_vulkan_interop",
        ":vulkan",
        "//cargo:sdl2",
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
