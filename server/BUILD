package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_library(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
        "//common:service_spec",
        "//common/service:srv_physics",
    ],
)

rust_binary(
    name = "launcher",
    srcs = ["launcher.rs"],
    deps = [
        ":server",
        "//common:init",
    ],
)

rust_library(
    name = "space",
    srcs = ["space.rs"],
    deps = [
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
        "//cargo/vendor:rand",
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
        "//cargo/vendor:chrono",
        "//cargo/vendor:log",
        "//cargo/vendor:rand",
    ],
)

rust_library(
    name = "space_partitioning",
    srcs = ["space_partitioning.rs"],
    deps = [],
)

rust_binary(
    name = "cosmic_toy",
    srcs = ["cosmic_toy.rs"],
    deps = [
        ":cosmic_physics",
        "//cargo/vendor:gnuplot",
        "//cargo/vendor:rand",
        "//cargo/vendor:vk_sys",
    ],
)

rust_test(
    name = "cosmic_physics_test",
    deps = [
        ":cosmic_physics",
    ],
)

rust_binary(
    name = "sdl2_toy",
    srcs = ["sdl2_toy.rs"],
    deps = [
        "//cargo/vendor:sdl2",
    ],
)
