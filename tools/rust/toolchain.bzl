load("@io_bazel_rules_rust//rust:repositories.bzl", "RUST_LINUX_BUILD_FILE", "DEFAULT_TOOLCHAINS")

def nightly_rust_repository():
  native.new_http_archive(
      name = "rust_linux_x86_64",
      url = "https://static.rust-lang.org/dist/2018-03-16/rust-nightly-x86_64-unknown-linux-gnu.tar.gz",
      strip_prefix = "rust-nightly-x86_64-unknown-linux-gnu",
      sha256 = "c06a1b8260387a3779ff5396e5e9498dbd5b112c34ce72e17df6fe79735a2e48",
      build_file_content = RUST_LINUX_BUILD_FILE,
  )

  native.new_local_repository(
      name = "rust_default_toolchains",
      path = ".",
      build_file_content = DEFAULT_TOOLCHAINS)

  # Register toolchains
  native.register_toolchains("@rust_default_toolchains//:rust-linux-x86_64")
