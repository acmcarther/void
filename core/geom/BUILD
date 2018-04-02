package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "geom",
    srcs = ["geom.rs"],
    data = [
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
    ],
)

rust_library(
    name = "geom_vk_support",
    srcs = ["geom_vk_support.rs"],
    data = [
    ],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        ":geom",
        "//core/rend/vk:vk_buffer_cache",
        "//core/rend/vk:vk_buffer_support",
        "//core/rend/vk:vk_lite",
        "//third_party/cargo:memoffset",
        "//third_party/cargo:vk_sys",
    ],
)
