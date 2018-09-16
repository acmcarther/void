package(default_visibility = ["//visibility:public"])

licenses([
    "notice",  # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
)

rust_binary(
    name = "bindgen_runner",
    srcs = ["bindgen_runner.rs"],
    deps = [
        "//third_party/cargo:bindgen",
    ],
)

genrule(
    name = "bindgen",  # Dictated by libsqlite-sys
    srcs = [
        "wrapper.h",
    ],
    outs = ["bindgen.rs"],
    cmd = "RUST_BACKTRACE=1 $(location :bindgen_runner) $(location :wrapper.h) > $(location :bindgen.rs)",
    tools = [
        ":bindgen_runner",
    ],
)

load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

pkg_tar(
    name = "bindgen_tar",
    srcs = [":bindgen"],
    extension = "tar.gz",
)

rust_library(
    name = "libsqlite3_sys",
    srcs = glob(["**/*.rs"]),
    crate_features = [
        "default",
        "min_sqlite_version_3_6_8",
        "pkg-config",
        "vcpkg",
    ],
    crate_root = "src/lib.rs",
    crate_type = "lib",
    out_dir_tar = ":bindgen_tar",
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    deps = [
        "//third_party/sqlite3",
    ],
)
