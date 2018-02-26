package(default_visibility = [
    "//experimental:__subpackages__"
])


load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "cosmic_physics",
    srcs = ["cosmic_physics.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:rand",
    ],
)

rust_library(
    name = "fast_cosmic_physics",
    srcs = ["fast_cosmic_physics.rs"],
    rustc_flags = [
        "-C opt-level=2",
    ],
    deps = [
        "//core/alg:octree",
        "//third_party/cargo:chrono",
        "//third_party/cargo:log",
        "//third_party/cargo:rand",
    ],
)

rust_binary(
    name = "cosmic_toy",
    srcs = ["cosmic_toy.rs"],
    deps = [
        ":cosmic_physics",
        "//third_party/cargo:gnuplot",
        "//third_party/cargo:rand",
        "//third_party/cargo:vk_sys",
    ],
)

rust_test(
    name = "cosmic_physics_test",
    deps = [
        ":cosmic_physics",
    ],
)
