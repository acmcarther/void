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
