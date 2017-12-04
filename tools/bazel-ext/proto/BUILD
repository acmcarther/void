package(default_visibility = ["//visibility:public"])

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_language")

proto_language(
    name = "rust",
    pb_plugin = "@protoc_gen_rust//:protoc_gen_rust",
    pb_file_extensions = [".rs"],
    output_to_workspace = False,
    pb_options = [],
    pb_imports = [],
)
