## Fetch overrides for cargo crates

# Override rust protobuf to fix generated file directory
# https://github.com/stepancheg/rust-protobuf/issues/189
new_git_repository(
    name = "custom_rust_protobuf",
    build_file = "third_party/custom/rust_protobuf.BUILD",
    commit = "1cd65f58202fd5ab66c9bf60e8194692d8a59313",
    remote = "https://github.com/acmcarther/rust-protobuf.git",
)

# Override rust grpc compiler for the same reason
new_git_repository(
    name = "custom_rust_grpc_compiler",
    build_file = "third_party/custom/rust_grpc_compiler.BUILD",
    commit = "121cd4534004fa8d80845f6629698a6032e38d49",
    remote = "https://github.com/acmcarther/grpc-rs.git",
)

## Fetch GRPC (old version, due to rust grpc incompat)
# https://github.com/pingcap/grpc-rs/issues/110
new_git_repository(
    name = "com_github_grpc_grpc",
    build_file = "third_party/grpc.BUILD",
    remote = "https://github.com/grpc/grpc",
    tag = "v1.7.2",
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
    commit = "88022d175adb48aa5f8904f95dfc716c543b3f1e",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

#load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
#rust_repositories()
load("//tools/rust:toolchain.bzl", "nightly_rust_repository")

nightly_rust_repository()

load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()

# TODO(acmcarther): Bring into repo
new_local_repository(
    name = "llvm",
    build_file = "//third_party:llvm.BUILD",
    #path = "/usr/lib",
    path = "/usr/lib/llvm-3.9/lib",
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
    #path = "/usr/lib/x86_64-linux-gnu",
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

new_git_repository(
    name = "reliable_io",
    build_file = "//third_party:reliable_io.BUILD",
    commit = "129451dd65853065076a4a606b0844d53859605e",
    remote = "https://github.com/networkprotocol/reliable.io",
)
