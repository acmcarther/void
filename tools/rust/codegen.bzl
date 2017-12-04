load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

def _gen_rust_library(ctx):
  lib_rs = ctx.attr.lib_rs
  deps = ctx.attr.deps

  lib_rs_file = ctx.new_file("lib.rs")
  ctx.file_action(
      output = lib_rs_file,
      content = lib_rs
  )


gen_rust_library = rule(
    implementation = _gen_rust_library,
    attrs = {
        "lib_rs": attr.string(),
        "srcs": attr.label_list(),
        "deps": attr.label_list(),
    }
)
