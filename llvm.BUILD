package(default_visibility = ["//visibility:public"])

licenses(["notice"])

alias(
    name = "clang-bin",
    actual = "bin/clang"
)

# Debian: apt-get install llvm-3.9
cc_library(
    name = "clang",
    srcs = [
      "lib/libclang.so",
      "lib/libclangAST.a",
      "lib/libclangAnalysis.a",
      "lib/libclangBasic.a",
      "lib/libclangDriver.a",
      "lib/libclangEdit.a",
      "lib/libclangFrontend.a",
      "lib/libclangIndex.a",
      "lib/libclangLex.a",
      "lib/libclangParse.a",
      "lib/libclangRewrite.a",
      "lib/libclangSema.a",
      "lib/libclangSerialization.a",
    ],
)
