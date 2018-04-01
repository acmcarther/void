package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "vk_lite",
    srcs = ["vk_lite.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//third_party/cargo:log",
        "//third_party/cargo:dylib",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_device_support",
    srcs = ["vk_device_support.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_lite",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:log",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_instance_support",
    srcs = ["vk_instance_support.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_lite",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:log",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_swapchain_support",
    srcs = ["vk_swapchain_support.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_device_support",
        ":vk_lite",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:log",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_pipeline_support",
    srcs = ["vk_pipeline_support.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_device_support",
        ":vk_lite",
        ":vk_swapchain_support",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:log",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_buffer_support",
    srcs = ["vk_buffer_support.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_device_support",
        ":vk_lite",
        ":vk_swapchain_support",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:log",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_descriptor_support",
    srcs = ["vk_descriptor_support.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_lite",
        "//third_party/cargo:derive_builder",
        "//third_party/cargo:log",
        "//third_party/cargo:vk_sys",
    ],
)

rust_library(
    name = "vk_buffer_cache",
    srcs = ["vk_buffer_cache.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_lite",
        ":vk_buffer_support",
        "//third_party/cargo:vk_sys",
    ],
)


rust_library(
    name = "sdl2_vulkan_interop",
    srcs = ["sdl2_vulkan_interop.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":vk_lite",
        "//third_party/cargo:sdl2",
        "//third_party/cargo:vk_sys",
        "//third_party/cargo/overrides/sdl2-sys:sdl2_sys",
    ],
)

rust_library(
    name = "sdl2_win",
    srcs = ["sdl2_win.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//third_party/cargo:sdl2",
    ],
)

rust_library(
    name = "vk_renderer",
    srcs = ["vk_renderer.rs"],
    deps = [
      ":vk_device_support",
      ":vk_instance_support",
      ":vk_lite",
      ":vk_pipeline_support",
      ":vk_swapchain_support",
       "//third_party/cargo:vk_sys",
       "//third_party/cargo:log",
    ]
)

