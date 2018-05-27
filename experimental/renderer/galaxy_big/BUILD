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
        "//core/geom",
        "//core/geom/gen:icosphere",
        "//core/rend/vk:sdl2_vulkan_interop",
        "//core/rend/vk:vk_base_renderer",
        "//core/rend/vk:vk_lite",
        "//core/spect:easy_spect",
        "//experimental/physics:fast_cosmic_physics",
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
        "//core/geom",
        "//core/geom/gen:icosphere",
        "//core/rend/vk:vk_base_renderer",
        "//core/rend/vk:vk_buffer_cache",
        "//core/rend/vk:vk_buffer_support",
        "//core/rend/vk:vk_descriptor_support",
        "//core/rend/vk:vk_device_support",
        "//core/rend/vk:vk_instance_support",
        "//core/rend/vk:vk_lite",
        "//core/rend/vk:vk_pipeline_support",
        "//core/rend/vk:vk_swapchain_support",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:vk_sys",
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
