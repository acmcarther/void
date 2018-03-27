package(default_visibility = ["//visibility:private"])

load("//tools:glslang.bzl", "compile_spirv_shader")
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "planet",
    srcs = ["planet.rs"],
    data = [
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":planet_renderer",
        "//core:init",
        "//core/rend/vk:renderer",
        "//core/rend/vk:sdl2_vulkan_interop",
        "//core/rend/vk:vk_lite",
        "//experimental/physics:cosmic_physics",
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
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
    name = "planet_renderer",
    srcs = ["planet_renderer.rs"],
    data = [
        ":planet_frag_shader",
        ":planet_vert_shader",
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//core/rend/vk:renderer",
        "//core/rend/vk:vk_buffer_support",
        "//core/rend/vk:vk_descriptor_support",
        "//core/rend/vk:vk_device_support",
        "//core/rend/vk:vk_instance_support",
        "//core/rend/vk:vk_lite",
        "//core/rend/vk:vk_pipeline_support",
        "//core/rend/vk:vk_swapchain_support",
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:vk_sys",
    ],
)

compile_spirv_shader(
    name = "planet_vert_shader",
    src = ":planet.vert",
)

compile_spirv_shader(
    name = "planet_frag_shader",
    src = ":planet.frag",
)
