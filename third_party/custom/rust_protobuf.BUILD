"""
Overridded version of the rust proto compiler plugin.

This fixes the output path of generated files, which in the original version are not relative to the proto file itself. This does not match up with other languages and makes using them in bazel a nuisance

See https://github.com/stepancheg/rust-protobuf/issues/189
"""
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "coded_input_stream" with type "bench" omitted
# Unsupported target "coded_output_stream" with type "bench" omitted

rust_library(
    name = "protobuf",
    crate_root = "protobuf/src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "-C opt-level=2",
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_protoc_gen_rust",
    crate_root = "protobuf/protoc-gen-rust.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":protobuf",
    ],
    rustc_flags = [
        "-C opt-level=2",
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

