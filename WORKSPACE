git_repository(
    name = "io_bazel_rules_rust",
    commit = "5b94fdb",
    remote = "https://github.com/acmcarther/rules_rust.git",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

git_repository(
    name = "org_pubref_rules_protobuf",
    commit = "563b674",
    remote = "https://github.com/pubref/rules_protobuf",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_repositories")

proto_repositories()

load("@org_pubref_rules_protobuf//cpp:rules.bzl", "cpp_proto_repositories")

cpp_proto_repositories()

new_git_repository(
    name = "protoc_gen_rust",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
)

rust_library(
  name = "protobuf",
  srcs = glob(["src/lib/**/*.rs"]),
  crate_root = "src/lib/protobuf.rs",
)
rust_binary(
  name = "protoc_gen_rust",
  srcs = ["src/protoc-gen-rust.rs"],
  crate_root = "src/protoc-gen-rust.rs",
  deps = [":protobuf"],
)""",
    commit = "f3af1e3",
    remote = "https://github.com/acmcarther/rust-protobuf",
)
