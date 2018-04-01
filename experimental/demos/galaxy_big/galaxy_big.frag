#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 inNorm;

layout(location = 0) out vec4 outColor;

void main() {
  outColor = vec4(inNorm, 1.0);
}
