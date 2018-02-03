new_git_repository(
    name = "custom_rust_protobuf",
    commit = "1cd65f58202fd5ab66c9bf60e8194692d8a59313",
    remote = "https://github.com/acmcarther/rust-protobuf.git",
    build_file = "custom_rust_protobuf.BUILD"
)

new_git_repository(
    name = "custom_rust_grpc_compiler",
    commit = "121cd4534004fa8d80845f6629698a6032e38d49",
    remote = "https://github.com/acmcarther/grpc-rs.git",
    build_file = "custom_rust_grpc_compiler.BUILD"
)

new_git_repository(
    name = "com_github_grpc_grpc",
    tag = "v1.7.2",
    remote = "https://github.com/grpc/grpc",
    build_file = "third_party/grpc.BUILD",
)
bind(
    name = "nanopb",
    actual = "//third_party/nanopb",
)

bind(
    name = "libssl",
    actual = "@boringssl//:ssl",
)

bind(
    name = "zlib",
    actual = "@submodule_zlib//:z",
)

bind(
    name = "protobuf",
    actual = "@com_google_protobuf//:protobuf",
)

bind(
    name = "protobuf_clib",
    actual = "@com_google_protobuf//:protoc_lib",
)

bind(
    name = "protocol_compiler",
    actual = "@com_google_protobuf//:protoc",
)

bind(
    name = "cares",
    actual = "@submodule_cares//:ares",
)

bind(
    name = "gtest",
    actual = "@submodule_gtest//:gtest",
)

bind(
    name = "gmock",
    actual = "@submodule_gtest//:gmock",
)

bind(
    name = "benchmark",
    actual = "@submodule_benchmark//:benchmark",
)

bind(
    name = "gflags",
    actual = "@com_github_gflags_gflags//:gflags",
)

local_repository(
    name = "boringssl",
    path = "third_party/boringssl-with-bazel",
)

new_local_repository(
    name = "submodule_zlib",
    build_file = "third_party/zlib.BUILD",
    path = "third_party/zlib",
)

new_local_repository(
    name = "com_google_protobuf",
    build_file = "third_party/protobuf/BUILD",
    path = "third_party/protobuf",
)

new_local_repository(
    name = "submodule_gtest",
    build_file = "third_party/gtest.BUILD",
    path = "third_party/googletest",
)

local_repository(
    name = "com_github_gflags_gflags",
    path = "third_party/gflags",
)

new_local_repository(
    name = "submodule_benchmark",
    path = "third_party/benchmark",
    build_file = "third_party/benchmark.BUILD",
)

new_local_repository(
    name = "submodule_cares",
    path = "third_party/cares",
    build_file = "third_party/cares/cares.BUILD",
)

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
