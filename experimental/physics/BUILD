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

rust_test(
    name = "cosmic_physics_test",
    deps = [
        ":cosmic_physics",
    ],
)

rust_test(
    name = "fast_cosmic_physics_test",
    deps = [
        ":fast_cosmic_physics",
    ],
)
