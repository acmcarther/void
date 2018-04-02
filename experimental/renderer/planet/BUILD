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
        "//core/geom:geometry",
        "//core/geom/gen:icosphere",
        "//core/rend/vk:vk_buffer_cache",
        "//core/rend/vk:vk_buffer_support",
        "//core/rend/vk:vk_descriptor_support",
        "//core/rend/vk:vk_device_support",
        "//core/rend/vk:vk_instance_support",
        "//core/rend/vk:vk_lite",
        "//core/rend/vk:vk_pipeline_support",
        "//core/rend/vk:vk_renderer",
        "//core/rend/vk:vk_swapchain_support",
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
        "//core/geom:geometry",
        "//core/geom/gen:icosphere",
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
