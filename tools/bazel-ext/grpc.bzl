# Essentially a repackaging of the new grpc_deps function from recent GRPC
# This was necessary as the rust grpc implementation doesn't directly support anything newer than 1.7.3
def old_grpc_repositories():
  native.bind(
      name = "nanopb",
      actual = "//third_party/nanopb",
  )

  native.bind(
      name = "libssl",
      actual = "@boringssl//:ssl",
  )

  native.bind(
      name = "zlib",
      actual = "@submodule_zlib//:z",
  )

  native.bind(
      name = "protobuf",
      actual = "@com_google_protobuf//:protobuf",
  )

  native.bind(
      name = "protobuf_clib",
      actual = "@com_google_protobuf//:protoc_lib",
  )

  native.bind(
      name = "protocol_compiler",
      actual = "@com_google_protobuf//:protoc",
  )

  native.bind(
      name = "cares",
      actual = "@submodule_cares//:ares",
  )

  native.bind(
      name = "gtest",
      actual = "@submodule_gtest//:gtest",
  )

  native.bind(
      name = "gmock",
      actual = "@submodule_gtest//:gmock",
  )

  native.bind(
      name = "benchmark",
      actual = "@submodule_benchmark//:benchmark",
  )

  native.bind(
      name = "gflags",
      actual = "@com_github_gflags_gflags//:gflags",
  )

  native.local_repository(
      name = "boringssl",
      path = "third_party/boringssl-with-bazel",
  )

  native.new_local_repository(
      name = "submodule_zlib",
      build_file = "third_party/zlib.BUILD",
      path = "third_party/zlib",
  )

  native.new_local_repository(
      name = "com_google_protobuf",
      build_file = "third_party/protobuf/BUILD",
      path = "third_party/protobuf",
  )

  native.new_local_repository(
      name = "submodule_gtest",
      build_file = "third_party/gtest.BUILD",
      path = "third_party/googletest",
  )

  native.local_repository(
      name = "com_github_gflags_gflags",
      path = "third_party/gflags",
  )

  native.new_local_repository(
      name = "submodule_benchmark",
      path = "third_party/benchmark",
      build_file = "third_party/benchmark.BUILD",
  )

  native.new_local_repository(
      name = "submodule_cares",
      path = "third_party/cares",
      build_file = "third_party/cares/cares.BUILD",
  )

