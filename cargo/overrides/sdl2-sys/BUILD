package(default_visibility = ["//visibility:public"])

load("//tools/rust:bindgen.bzl", "bindgen")

bindgen(
    name = "SDL_bindgen",
    blacklist_types = [
        "FP_NAN",
        "FP_INFINITE",
        "FP_ZERO",
        "FP_SUBNORMAL",
        "FP_NORMAL",
        "max_align_t",
        "_",
    ],
    clang_args = [
        "-Ithird_party/SDL2/include",
    ],
    flags = [
        # Backcompat with upstream sdl2-sys
        "--rustified-enum \".*\"",
    ],
    hdr = ":wrapper.h",
    includes = ["//third_party/SDL2:sdl2_headers"],
)

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "sdl2_sys",
    srcs = [":SDL_bindgen.rs"],
    crate_features = [
        "default",
    ],
    crate_root = "SDL_bindgen.rs",
    crate_type = "lib",
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    deps = [
        "//third_party/SDL2",
    ],
)
