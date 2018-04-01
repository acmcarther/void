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
    name = "renderer",
    srcs = ["renderer.rs"],
    data = [
        ":frag_shader",
        ":vert_shader",
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
        "//game/client/renderer:vk_buffer_cache",
        "//game/client/renderer:vk_buffer_support",
        "//game/client/renderer:vk_descriptor_support",
        "//game/client/renderer:vk_device_support",
        "//game/client/renderer:vk_instance_support",
        "//game/client/renderer:vk_lite",
        "//game/client/renderer:vk_pipeline_support",
        "//game/client/renderer:vk_renderer",
        "//game/client/renderer:vk_swapchain_support",
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
        ":renderer",
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:rand",
    ],
)

rust_test(
    name = "planet_test",
    deps = [":planet"],
)

load("//tools:glslang.bzl", "compile_spirv_shader")

compile_spirv_shader(
    name = "vert_shader",
    src = ":vert.vert",
)

compile_spirv_shader(
    name = "frag_shader",
    src = ":frag.frag",
)
