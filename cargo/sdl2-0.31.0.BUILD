"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

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
        "@raze__bitflags__0_7_0//:bitflags",
        "@raze__lazy_static__0_2_11//:lazy_static",
        "@raze__libc__0_2_34//:libc",
        "@raze__num__0_1_41//:num",
        "@raze__rand__0_3_18//:rand",
        "@//third_party/SDL2:sdl2_sys",
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
