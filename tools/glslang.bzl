def compile_spirv_shader(name, src):
  native.genrule(
      name = name,
      srcs = [ src ],
      outs = [name + ".spv"],
      cmd = "$(location @glslang//:glslangValidator) -V $(location " + src + ") -o $@",
      tools = ["@glslang//:glslangValidator"])
