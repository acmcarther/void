## Fetch overrides for cargo crates

# Override rust protobuf to fix generated file directory
# https://github.com/stepancheg/rust-protobuf/issues/189
new_git_repository(
    name = "custom_rust_protobuf",
    commit = "1cd65f58202fd5ab66c9bf60e8194692d8a59313",
    remote = "https://github.com/acmcarther/rust-protobuf.git",
    build_file = "custom_rust_protobuf.BUILD"
)

# Override rust grpc compiler for the same reason
new_git_repository(
    name = "custom_rust_grpc_compiler",
    commit = "121cd4534004fa8d80845f6629698a6032e38d49",
    remote = "https://github.com/acmcarther/grpc-rs.git",
    build_file = "custom_rust_grpc_compiler.BUILD"
)

## Fetch GRPC (old version, due to rust grpc incompat)
# https://github.com/pingcap/grpc-rs/issues/110
new_git_repository(
    name = "com_github_grpc_grpc",
    tag = "v1.7.2",
    remote = "https://github.com/grpc/grpc",
    build_file = "third_party/grpc.BUILD",
)

load("//tools/bazel-ext:grpc.bzl", "old_grpc_repositories")
old_grpc_repositories()

git_repository(
    name = "org_pubref_rules_protobuf",
    commit = "91daba1e655805180744cd5a86a6c0095d280b65",
    remote = "https://github.com/pubref/rules_protobuf",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_repositories")
proto_repositories()

git_repository(
    name = "io_bazel_rules_rust",
    commit = "5bc46ddca8817072cdae1961b3f9830a2bc3afa7",
    remote = "https://github.com/acmcarther/rules_rust.git",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("//cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()

# TODO(acmcarther): Bring into repo
new_local_repository(
    name = "llvm",
    build_file = "//third_party:llvm.BUILD",
    path = "/usr/lib",
)

# TODO(acmcarther): Bring into repo
new_local_repository(
    name = "clang",
    build_file = "//third_party:clang.BUILD",
    path = "/usr/lib",
)

# TODO(acmcarther): Bring into repo
new_local_repository(
    name = "libsodium",
    build_file = "//third_party:libsodium.BUILD",
    path = "/usr/lib",
)

new_http_archive(
    name = "glslang",
    build_file = "//third_party:glslang.BUILD",
    type = "zip",
    url = "https://github.com/KhronosGroup/glslang/releases/download/master-tot/glslang-master-linux-Release.zip",
)

new_git_repository(
    name = "netcode_io",
    build_file = "//third_party:netcode_io.BUILD",
    commit = "b261a1da3ac6eac9a7df53ca2de95959c24de158",
    remote = "https://github.com/networkprotocol/netcode.io",
)

