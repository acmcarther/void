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
# Unsupported target "animation" with type "example" omitted
# Unsupported target "audio" with type "test" omitted
# Unsupported target "audio-capture-and-replay" with type "example" omitted
# Unsupported target "audio-queue-squarewave" with type "example" omitted
# Unsupported target "audio-squarewave" with type "example" omitted
# Unsupported target "audio-wav" with type "example" omitted
# Unsupported target "audio-whitenoise" with type "example" omitted
# Unsupported target "demo" with type "example" omitted
# Unsupported target "events" with type "test" omitted
# Unsupported target "game-controller" with type "example" omitted
# Unsupported target "game-of-life" with type "example" omitted
# Unsupported target "game-of-life-unsafe-textures" with type "example" omitted
# Unsupported target "gfx-demo" with type "example" omitted
# Unsupported target "haptic" with type "example" omitted
# Unsupported target "image-demo" with type "example" omitted
# Unsupported target "joystick" with type "example" omitted
# Unsupported target "keyboard-state" with type "example" omitted
# Unsupported target "message-box" with type "example" omitted
# Unsupported target "mixer-demo" with type "example" omitted
# Unsupported target "mouse-state" with type "example" omitted
# Unsupported target "no-renderer" with type "example" omitted
# Unsupported target "relative-mouse-state" with type "example" omitted
# Unsupported target "renderer-target" with type "example" omitted
# Unsupported target "renderer-texture" with type "example" omitted
# Unsupported target "renderer-yuv" with type "example" omitted
# Unsupported target "resource-manager" with type "example" omitted

rust_library(
    name = "sdl2",
    crate_root = "src/sdl2/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/bitflags-0.7.0:bitflags",
        "//cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//cargo/vendor/libc-0.2.34:libc",
        "//cargo/vendor/num-0.1.41:num",
        "//cargo/vendor/rand-0.3.18:rand",
        "//cargo/overrides/sdl2-sys-0.31.0:sdl2_sys",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
    ],
)

# Unsupported target "ttf-demo" with type "example" omitted
# Unsupported target "video" with type "test" omitted
# Unsupported target "window-properties" with type "example" omitted
