package(default_visibility = ["//visibility:private"])

load("//tools:glslang.bzl", "compile_spirv_shader")
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "galaxy_big",
    srcs = ["galaxy_big.rs"],
    data = [
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":galaxy_big_renderer",
        "//core:init",
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
        "//game/client/renderer:vk_renderer",
        "//game/client/renderer:sdl2_vulkan_interop",
        "//game/client/renderer:vk_lite",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:chrono",
        "//third_party/cargo:fern",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:png",
        "//third_party/cargo:rand",
        "//third_party/cargo:sdl2",
        "//third_party/cargo:zcfg",
        "//experimental/physics:fast_cosmic_physics",
    ],
)

rust_library(
    name = "galaxy_big_renderer",
    srcs = ["galaxy_big_renderer.rs"],
    data = [
        ":galaxy_big_frag_shader",
        ":galaxy_big_vert_shader",
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
        "//game/client/renderer:vk_renderer",
        "//game/client/renderer:vk_buffer_support",
        "//game/client/renderer:vk_descriptor_support",
        "//game/client/renderer:vk_device_support",
        "//game/client/renderer:vk_instance_support",
        "//game/client/renderer:vk_lite",
        "//game/client/renderer:vk_buffer_cache",
        "//game/client/renderer:vk_pipeline_support",
        "//game/client/renderer:vk_swapchain_support",
        "//third_party/cargo:vk_sys",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:log",
    ],
)

compile_spirv_shader(
    name = "galaxy_big_vert_shader",
    src = ":galaxy_big.vert",
)

compile_spirv_shader(
    name = "galaxy_big_frag_shader",
    src = ":galaxy_big.frag",
)
