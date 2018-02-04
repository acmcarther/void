package(default_visibility = ["//visibility:private"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "galaxy_toy",
    srcs = ["galaxy_toy.rs"],
    data = [
        ":explosion.png",
        ":star_frag_shader",
        ":star_vert_shader",
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//cargo:cgmath",
        "//cargo:chrono",
        "//cargo:fern",
        "//cargo:log",
        "//cargo:memoffset",
        "//cargo:png",
        "//cargo:rand",
        "//cargo:sdl2",
        "//cargo:vk_sys",
        "//client/renderer:sdl2_vulkan_interop",
        "//client/renderer:vk_buffer_support",
        "//client/renderer:vk_descriptor_support",
        "//client/renderer:vk_device_support",
        "//client/renderer:vk_instance_support",
        "//client/renderer:vk_lite",
        "//client/renderer:vk_pipeline_support",
        "//client/renderer:vk_swapchain_support",
        "//server/experimental:cosmic_physics",
    ],
)

load("//tools:glslang.bzl", "compile_spirv_shader")

compile_spirv_shader(
    name = "star_vert_shader",
    src = ":star.vert",
)

compile_spirv_shader(
    name = "star_frag_shader",
    src = ":star.frag",
)
