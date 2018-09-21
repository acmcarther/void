package(default_visibility = [
    "//experimental:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
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
        "//core/gfx:gfx_basics",
        "//core/gfx/procedural:icosphere",
        "//core/gfx/vk:vk_application",
        "//core/gfx/vk:vk_base_renderer",
        "//core/gfx/vk:vk_basics",
        "//core/gfx/vk:vk_buffer_cache",
        "//core/gfx/vk:vk_buffer_support",
        "//core/gfx/vk:vk_descriptor_support",
        "//core/gfx/vk:vk_device_support",
        "//core/gfx/vk:vk_instance_support",
        "//core/gfx/vk:vk_lite",
        "//core/gfx/vk:vk_pipeline_support",
        "//core/gfx/vk:vk_swapchain_support",
        "//third_party/cargo:cgmath",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "planet",
    srcs = ["planet.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":planet_renderer",
        "//core/gfx:gfx_basics",
        "//core/gfx/procedural:icosphere",
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:rand",
    ],
)

load("//tools:glslang.bzl", "compile_spirv_shader")

compile_spirv_shader(
    name = "planet_vert_shader",
    src = ":planet.vert",
)

compile_spirv_shader(
    name = "planet_frag_shader",
    src = ":planet.frag",
)
