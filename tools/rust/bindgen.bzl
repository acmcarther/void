def bindgen(name, hdr, includes=[], formatted=True, blacklist_types=[], flags = [], clang_args = []):
  bindgen_out_name = name
  if formatted:
    bindgen_out_name = name + "_unformatted"

  flags = flags + [
      "--no-rustfmt-bindings",
  ]
  for blacklist_type in blacklist_types:
    flags.append("--blacklist-type " + blacklist_type)

  flags = ' '.join(flags)

  clang_args = ' '.join(clang_args)

  native.genrule(
      name = bindgen_out_name,
      srcs = includes + [
          hdr,
      ],
      outs = [bindgen_out_name + ".rs"],
      cmd = "RUST_BACKTRACE=1 $(location //cargo:cargo_bin_bindgen) " + flags + " $(location " + hdr + ") > $(location " + bindgen_out_name + ".rs) -- " + clang_args,
      tools = [
          "//cargo:cargo_bin_bindgen",
      ])

  if formatted:
    native.genrule(
        name = name,
          srcs = [bindgen_out_name],
        outs = [name + ".rs"],
        cmd = "$(location //cargo/overrides/rustfmt-0.3.6:cargo_bin_rustfmt) --write-mode=plain $(location " + bindgen_out_name + ") > $@",
        tools = ["//cargo/overrides/rustfmt-0.3.6:cargo_bin_rustfmt"])
