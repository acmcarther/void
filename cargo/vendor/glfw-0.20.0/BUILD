"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//cargo:__subpackages__"])

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
        "//cargo/vendor/bitflags-1.0.1:bitflags",
        "//cargo/vendor/enum_primitive-0.1.1:enum_primitive",
        "//cargo/vendor/glfw-sys-3.2.2:glfw_sys",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/log-0.3.8:log",
        "//cargo/vendor/num-0.1.41:num",
        "//cargo/vendor/semver-0.2.3:semver",
        
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
