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
        "//core/gfx/vk:sdl2_vulkan_interop",
        "//core/gfx/vk:vk_buffer_support",
        "//core/gfx/vk:vk_descriptor_support",
        "//core/gfx/vk:vk_device_support",
        "//core/gfx/vk:vk_instance_support",
        "//core/gfx/vk:vk_lite",
        "//core/gfx/vk:vk_pipeline_support",
        "//core/gfx/vk:vk_swapchain_support",
        "//experimental/physics:cosmic_physics",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:chrono",
        "//third_party/cargo:fern",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:png",
        "//third_party/cargo:rand",
        "//third_party/cargo:sdl2",
        "//third_party/cargo:vk_sys",
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
