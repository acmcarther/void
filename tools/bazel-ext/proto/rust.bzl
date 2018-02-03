load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_language", "proto_compile")

def build_librs_cmd(deps, with_grpc):
  lib_rs = "echo \"extern crate protobuf;\" > $@"
  if with_grpc:
    lib_rs = lib_rs + "echo \"extern crate grpcio;\" >> $@"
    lib_rs = lib_rs + "echo \"extern crate futures;\" >> $@"
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
    # Add use statements for local modules.
    # TODO: Clean this up, its a hack for grpc
    for s_use in $(SRCS)
    do
      use_path_less_rs="$${s_use%%.rs}"
      use_path_less_tree="$${use_path_less_rs##*/}"
      if [ "$$use_path_less_tree" != "$$path_less_tree" ];
      then
        echo "use ::$$use_path_less_tree;\n" >> $@
      fi
    done
    echo "}\n" >> $@
  done
  """ % use_cmds

  return lib_rs


def rust_proto_library(
    name,
    protos = [],
    proto_deps = [],
    proto_dep_crates = [],
    with_grpc = False):

  proto_compile(
    name = name + ".pb",
    # Pass in a list of proto_language rules
    langs = ["//tools/bazel-ext/proto:rust"],
    deps = [dep + ".pb" for dep in proto_deps],
    protos = protos,
    with_grpc = with_grpc,
  )

  native.genrule(
    name = name + "lib_rs",
    srcs = [name + ".pb"],
    outs = [name + "_lib.rs"],
    # This is a pretty naive soln
    cmd = build_librs_cmd(proto_dep_crates, with_grpc)
  )

  builtin_deps = ["@custom_rust_protobuf//:protobuf"]
  if with_grpc:
    builtin_deps = builtin_deps + ["//cargo:grpcio", "//cargo:futures"]

  rust_library(
    name = name,
    srcs = [name + "lib_rs"],
    crate_root = name + "_lib.rs",
    deps = builtin_deps + proto_deps,
  )

