package(default_visibility = ["//visibility:public"])

licenses(["notice"])

filegroup(
    name = "sqlite3_headers",
    srcs = [
        "sqlite3.h",
        "sqlite3ext.h",
    ],
)

cc_library(
    name = "sqlite3",
    srcs = [
        "sqlite3.c",
    ],
    hdrs = [
        "sqlite3.h",
        "sqlite3ext.h",
    ],
    copts = [
        "-DSQLITE_CORE",
        "-DSQLITE_DEFAULT_FOREIGN_KEYS=1",
        "-DSQLITE_ENABLE_API_ARMOR",
        "-DSQLITE_ENABLE_COLUMN_METADATA",
        "-DSQLITE_ENABLE_DBSTAT_VTAB",
        "-DSQLITE_ENABLE_FTS3",
        "-DSQLITE_ENABLE_FTS3_PARENTHESIS",
        "-DSQLITE_ENABLE_FTS5",
        "-DSQLITE_ENABLE_JSON1",
        "-DSQLITE_ENABLE_LOAD_EXTENSION=1",
        "-DSQLITE_ENABLE_MEMORY_MANAGEMENT",
        "-DSQLITE_ENABLE_RTREE",
        "-DSQLITE_ENABLE_STAT2",
        "-DSQLITE_ENABLE_STAT4",
        "-DSQLITE_HAVE_ISNAN",
        "-DSQLITE_SOUNDEX",
        "-DSQLITE_THREADSAFE=1",
        "-DSQLITE_USE_URI",
        "-DHAVE_USLEEP=1",
    ],
    linkopts = [
        "-ldl",
        "-lpthread",
    ],
)

cc_binary(
    name = "sqlite3_shell",
    srcs = [
        "shell.c",
    ],
    deps = [
        ":sqlite3",
    ],
)

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
        ":sqlite3.h",
    ],
    outs = ["bindgen.rs"],
    cmd = "RUST_BACKTRACE=1 $(location :bindgen_runner) $(location :sqlite3.h) > $(location :bindgen.rs)",
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

