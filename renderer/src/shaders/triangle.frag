#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec4 curr_color;
layout (location = 1) in vec4 next_color;
layout (location = 0) out vec4 uFragColor;
layout (push_constant) uniform PushConstants {
    uint time;
} pcs;

void main() {
    float val = mod(pcs.time, 1000.0) / 1000.0;
    vec3 new_color = mix(curr_color.xyz, next_color.xyz, val);

    uFragColor = vec4(new_color, 1.0);
}
