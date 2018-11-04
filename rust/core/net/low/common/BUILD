package(default_visibility = [
    "//experimental:__subpackages__",
    "//rust/core/net/low:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "common",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//third_party/cargo:lazy_static",
        "//third_party/cargo:libc",
        "//third_party/cargo:log",
        "//third_party/cargo:zcfg",
        "//rust/core/base",
        "@netcode_io//:netcode_io_sys",
        "@reliable_io//:reliable_io_sys",
    ],
)
