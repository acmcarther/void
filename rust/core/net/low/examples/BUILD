package(default_visibility = [
    "//experimental:__subpackages__",
    "//rust/game/net:__subpackages__",
    "//rust/game/state/relay/demo:__subpackages__",
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)


rust_binary(
    name = "netcode_demo",
    srcs = ["netcode_demo.rs"],
    deps = [
        "//rust/core/net/low/common",
        "//rust/core/net/low/client",
        "//rust/core/net/low/server",
        "//rust/core/base",
        "//third_party/cargo:log",
        "@netcode_io//:netcode_io_sys",
    ],
)
