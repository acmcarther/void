load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_language", "proto_compile")

def build_librs_cmd(deps):
  lib_rs = "echo \"extern crate protobuf;\" > $@"
  for dep in deps:
    lib_rs = lib_rs + "echo \"extern crate %s;\" >> $@" % dep
  use_cmds ="".join(["echo \"use %s::*;\" >> $@" % dep for dep in deps])
  lib_rs = lib_rs + """
  for s in $(SRCS)
  do
    path_less_rs="$${s%%.rs}"
    path_less_tree="$${path_less_rs##*/}"
    echo "pub mod $$path_less_tree {\n" >> $@
    sed 's/super::*//g' $$s >> $@
    # Pipe use statements into the end of the mod to
    # prevent trying to use before #[allow]
    %s
    echo "}\n" >> $@
  done
  """ % use_cmds

  return lib_rs


proto_language(
    name = "rust",
    pb_plugin = "//cargo:cargo_bin_protoc_gen_rust",
    pb_file_extension = ["_lib.rs"],
    supports_grpc = True,
    grpc_plugin = "//cargo:cargo_bin_grpc_rust_plugin",
)
