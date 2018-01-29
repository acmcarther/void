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
    name = "vk_lite",
    srcs = ["vk_lite.rs"],
    deps = [
        "//cargo:dylib",
        "//cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_device_support",
    srcs = ["vk_device_support.rs"],
    deps = [
        "//cargo:derive_builder",
        "//cargo:vk_sys",
        ":vk_lite",
        "//cargo:log",
    ],
)

rust_library(
    name = "vk_instance_support",
    srcs = ["vk_instance_support.rs"],
    deps = [
        "//cargo:derive_builder",
        "//cargo:vk_sys",
        ":vk_lite",
        "//cargo:log",
    ],
)

rust_library(
    name = "vk_swapchain_support",
    srcs = ["vk_swapchain_support.rs"],
    deps = [
        "//cargo:derive_builder",
        "//cargo:vk_sys",
        ":vk_lite",
        ":vk_device_support",
        "//cargo:log",
    ],
)

rust_library(
    name = "vk_pipeline_support",
    srcs = ["vk_pipeline_support.rs"],
    deps = [
        "//cargo:derive_builder",
        "//cargo:vk_sys",
        ":vk_lite",
        ":vk_device_support",
        ":vk_swapchain_support",
        "//cargo:log",
    ],
)

rust_library(
    name = "vk_buffer_support",
    srcs = ["vk_buffer_support.rs"],
    deps = [
        "//cargo:derive_builder",
        "//cargo:vk_sys",
        ":vk_lite",
        ":vk_device_support",
        ":vk_swapchain_support",
        "//cargo:log",
    ],
)

rust_library(
    name = "vk_new_triangle",
    srcs = ["vk_new_triangle.rs"],
    data = [
        ":triangle_frag_shader",
        ":triangle_vert_shader",
        ":texture.png",
    ],
    deps = [
        "//cargo:vk_sys",
        "//cargo:memoffset",
        ":vk_lite",
        ":vk_device_support",
        ":vk_instance_support",
        ":vk_swapchain_support",
        ":vk_pipeline_support",
        ":vk_buffer_support",
        "//cargo:log",
        "//cargo:cgmath",
        "//cargo:chrono",
        "//cargo:png",
    ],
)


rust_library(
    name = "sdl2_vulkan_interop",
    srcs = ["sdl2_vulkan_interop.rs"],
    deps = [
        ":vk_lite",
        "//cargo:sdl2",
        "//cargo:vk_sys",
        "//cargo/overrides/sdl2-sys:sdl2_sys",
    ],
)

rust_binary(
    name = "toy",
    srcs = ["toy.rs"],
    deps = [
        ":sdl2_vulkan_interop",
        ":vk_lite",
        ":vk_new_triangle",
        "//cargo:sdl2",
        "//cargo:fern",
        "//cargo:chrono",
        "//cargo:log",
    ],
)

load("//tools:glslang.bzl", "compile_spirv_shader")

compile_spirv_shader(
    name = "triangle_vert_shader",
    src = ":triangle.vert",
)

compile_spirv_shader(
    name = "triangle_frag_shader",
    src = ":triangle.frag",
)
