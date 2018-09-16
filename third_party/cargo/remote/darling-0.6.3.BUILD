"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//third_party/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "accrue_errors" with type "test" omitted
# Unsupported target "automatic_bounds" with type "example" omitted
# Unsupported target "computed_bound" with type "test" omitted
# Unsupported target "consume_fields" with type "example" omitted
# Unsupported target "custom_bound" with type "test" omitted

rust_library(
    name = "darling",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__darling_core__0_6_3//:darling_core",
        "@raze__darling_macro__0_6_3//:darling_macro",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.6.3",
    crate_features = [
    ],
)

# Unsupported target "enums_newtype" with type "test" omitted
# Unsupported target "enums_struct" with type "test" omitted
# Unsupported target "enums_unit" with type "test" omitted
# Unsupported target "error" with type "test" omitted
# Unsupported target "fallible_read" with type "example" omitted
# Unsupported target "from_generics" with type "test" omitted
# Unsupported target "from_type_param" with type "test" omitted
# Unsupported target "from_type_param_default" with type "test" omitted
# Unsupported target "from_variant" with type "test" omitted
# Unsupported target "generics" with type "test" omitted
# Unsupported target "happy_path" with type "test" omitted
# Unsupported target "multiple" with type "test" omitted
# Unsupported target "newtype" with type "test" omitted
# Unsupported target "skip" with type "test" omitted
# Unsupported target "split_declaration" with type "test" omitted
# Unsupported target "supports" with type "test" omitted
# Unsupported target "supports_struct" with type "example" omitted
