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


# Unsupported target "cursor" with type "example" omitted
# Unsupported target "fullscreen" with type "example" omitted
# Unsupported target "grabbing" with type "example" omitted
# Unsupported target "min_max_size" with type "example" omitted
# Unsupported target "multiwindow" with type "example" omitted
# Unsupported target "proxy" with type "example" omitted
# Unsupported target "send_objects" with type "test" omitted
# Unsupported target "transparent" with type "example" omitted
# Unsupported target "window" with type "example" omitted
rust_library(
    name = "winit",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/wayland-client-0.12.2:wayland_client",
        "//cargo/vendor/wayland-kbd-0.13.0:wayland_kbd",
        "//cargo/vendor/wayland-protocols-0.12.2:wayland_protocols",
        "//cargo/vendor/wayland-window-0.13.2:wayland_window",
        "//cargo/vendor/x11-dl-2.17.0:x11_dl",
        
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
        
    ],
    
    crate_features = [
        
    ],
)
