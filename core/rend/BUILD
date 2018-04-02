package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
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
