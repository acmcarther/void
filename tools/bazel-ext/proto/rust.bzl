load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)
load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")

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

def rust_proto_library(
    name,
    protos = [],
    srcs = [],
    proto_deps = [],
    proto_dep_crates = []):

  proto_dep_uses = ["use {};".format(dep) for dep in proto_dep_crates]

  proto_compile(
    name = name + ".pb",
    # Pass in a list of proto_language rules
    langs = ["//tools/bazel-ext/proto:rust"],
    deps = [dep + ".pb" for dep in proto_deps],
    protos = protos
  )

  native.genrule(
    name = name + "lib_rs",
    srcs = [name + ".pb"],
    outs = [name + "_lib.rs"],
    # This is a pretty naive soln
    cmd = build_librs_cmd(proto_dep_crates)
  )

  rust_library(
    name = name,
    srcs = [name + "lib_rs"],
    crate_root = name + "_lib.rs",
    deps = ["@protoc_gen_rust//:protobuf",] + proto_deps,
  )
