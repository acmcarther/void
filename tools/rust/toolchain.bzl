load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")

def nightly_rust_repository():
  rust_repository_set(
      name = "rust_default_toolchains",
      version = "nightly",
      exec_triple = "x86_64-unknown-linux-gnu",
      extra_target_triples = [],
      iso_date = "2018-08-31",
  )
