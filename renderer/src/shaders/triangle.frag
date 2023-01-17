#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec4 o_color;
layout (location = 0) out vec4 uFragColor;
layout (push_constant) uniform PushConstants {
    uint time;
} pcs;

void main() {
    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos((pcs.time / 500.0) + o_color.xyx);
    uFragColor = vec4(col, 1.0);
}
