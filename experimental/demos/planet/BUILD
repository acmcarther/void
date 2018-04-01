package(default_visibility = ["//visibility:private"])

load("//tools:glslang.bzl", "compile_spirv_shader")
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "bin",
    srcs = ["bin.rs"],
    data = [
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//core:init",
        "//game/client/renderer:sdl2_win",
        "//third_party/cargo:log",
        "//third_party/cargo:sdl2",
        ":sim",
        ":renderer",
    ],
)

rust_library(
    name = "renderer",
    srcs = ["renderer.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//game/assets:geometry",
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
    ],
)

rust_library(
    name = "sim",
    srcs = ["sim.rs"],
    data = [
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//game/assets:geometry",
        "//game/assets/gen:icosphere",
        "//third_party/cargo:chrono",
        "//third_party/cargo:fern",
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:log",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:png",
        "//third_party/cargo:rand",
        "//third_party/cargo:zcfg",
    ],
)
