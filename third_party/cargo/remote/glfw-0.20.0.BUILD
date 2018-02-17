"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "clipboard" with type "example" omitted
# Unsupported target "cursor" with type "example" omitted
# Unsupported target "cursor_icon" with type "example" omitted
# Unsupported target "defaults" with type "example" omitted
# Unsupported target "error" with type "example" omitted
# Unsupported target "events" with type "example" omitted
# Unsupported target "fullscreen" with type "example" omitted

rust_library(
    name = "glfw",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__bitflags__1_0_1//:bitflags",
        "@raze__enum_primitive__0_1_1//:enum_primitive",
        "@raze__glfw_sys__3_2_2//:glfw_sys",
        "@raze__libc__0_2_36//:libc",
        "@raze__log__0_3_9//:log",
        "@raze__num__0_1_41//:num",
        "@raze__semver__0_2_3//:semver",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "glfw-sys",
    ],
)

# Unsupported target "modes" with type "example" omitted
# Unsupported target "monitors" with type "example" omitted
# Unsupported target "multiwindow" with type "example" omitted
# Unsupported target "render_task" with type "example" omitted
# Unsupported target "title" with type "example" omitted
# Unsupported target "version" with type "example" omitted
# Unsupported target "vulkan" with type "example" omitted
# Unsupported target "window" with type "example" omitted
# Unsupported target "window_icon" with type "example" omitted
