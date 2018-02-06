package(default_visibility = ["//visibility:public"])

licenses(["notice"])

cc_library(
    name = "netcode_io",
    srcs = [
      "netcode.c",
      "netcode.h",
    ],
    deps = [
      "@libsodium//:sodium",
    ]
)

load("@//tools/rust:bindgen.bzl", "bindgen")

bindgen(
    name = "netcode_io_bindgen",
    hdr = ":netcode.h",
)

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "netcode_io_sys",
    srcs = [":netcode_io_bindgen.rs"],
    deps = [
      ":netcode_io",
    ],
    crate_features = [
    ],
    crate_root = "netcode_io_bindgen.rs",
    crate_type = "lib",
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
)
