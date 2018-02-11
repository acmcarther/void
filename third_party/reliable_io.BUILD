package(default_visibility = ["//visibility:public"])

licenses(["notice"])

cc_library(
    name = "reliable_io",
    srcs = [
      "reliable.c",
      "reliable.h",
    ],
    deps = []
)

load("@//tools/rust:bindgen.bzl", "bindgen")

bindgen(
    name = "reliable_io_bindgen",
    hdr = ":reliable.h",
)

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "reliable_io_sys",
    srcs = [":reliable_io_bindgen.rs"],
    deps = [
      ":reliable_io",
    ],
    crate_features = [
    ],
    crate_root = "reliable_io_bindgen.rs",
    crate_type = "lib",
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
)
