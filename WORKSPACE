git_repository(
    name = "com_github_grpc_grpc",
    tag = "v1.9.0",
    remote = "https://github.com/grpc/grpc"
)

load("@com_github_grpc_grpc//bazel:grpc_deps.bzl", "grpc_deps")
grpc_deps()

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
    build_file = "llvm.BUILD",
    path = "/usr/lib",
)

# TODO(acmcarther): Bring into repo
new_local_repository(
    name = "clang",
    build_file = "clang.BUILD",
    path = "/usr/lib",
)


new_http_archive(
    name = "glslang",
    build_file = "//third_party:glslang.BUILD",
    type = "zip",
    url = "https://github.com/KhronosGroup/glslang/releases/download/master-tot/glslang-master-linux-Release.zip",
)
