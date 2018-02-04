package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "space",
    srcs = ["space.rs"],
    deps = [
        "//cargo:chrono",
        "//cargo:log",
        "//cargo:rand",
    ],
)

rust_test(
    name = "space_test",
    deps = [
        ":space",
    ],
)

rust_library(
    name = "cosmic_physics",
    srcs = ["cosmic_physics.rs"],
    deps = [
        "//cargo:chrono",
        "//cargo:log",
        "//cargo:rand",
    ],
)

rust_binary(
    name = "cosmic_toy",
    srcs = ["cosmic_toy.rs"],
    deps = [
        ":cosmic_physics",
        "//cargo:gnuplot",
        "//cargo:rand",
        "//cargo:vk_sys",
    ],
)

rust_test(
    name = "cosmic_physics_test",
    deps = [
        ":cosmic_physics",
    ],
)
