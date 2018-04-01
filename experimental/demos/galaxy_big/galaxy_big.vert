#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(push_constant) uniform Model {
  mat4 model;
} model;

layout(binding = 0) uniform MVPUniform {
  mat4 view;
  mat4 proj;
} mvpUbo;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inNorm;

layout(location = 0) out vec3 outNorm;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = mvpUbo.proj * mvpUbo.view * model.model * vec4(inPosition, 1.0);
    outNorm = inNorm;
}
