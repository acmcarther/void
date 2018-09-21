package(default_visibility = ["//visibility:public"])

licenses([
    "notice",  # "MIT"
])


load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
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
    out_dir_tar = "//third_party/sqlite3:bindgen_tar",
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    deps = [
        "//third_party/sqlite3",
    ],
)
