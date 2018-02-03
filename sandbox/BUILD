package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
    "rust_binary",
)

rust_binary(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        "//cargo:futures",
        "//cargo:grpcio",
        "//cargo:grpcio_proto",
        "//cargo:log",
    ],
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_language", "proto_compile")

proto_language(
    name = "rust",
    pb_plugin = "@custom_rust_protobuf//:cargo_bin_protoc_gen_rust",
    pb_file_extensions = [".rs"],
    supports_grpc = True,
    grpc_file_extensions = ["_grpc.rs"],
    grpc_plugin = "@custom_rust_grpc_compiler//:cargo_bin_grpc_rust_plugin",
    pb_imports = [],
    output_to_workspace = True,
)

proto_compile(
    name = "helloworld_rs",
    langs = [
        ":rust",
    ],
    with_grpc = True,
    args = [],
    protos = [
        ":helloworld.proto",
    ],
    verbose = 3
)
