package(default_visibility = ["//visibility:public"])

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_language")

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
