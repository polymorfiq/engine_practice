#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

// layout (location = 0) in vec4 curr_color;
// layout (location = 1) in vec4 next_color;
layout (location = 0) out vec4 uFragColor;
layout(binding = 1) uniform Material {
    uint id;
} material;

layout (push_constant) uniform PushConstants {
    uint time;
} pcs;

void main() {
    if (material.id == 1) {
        uFragColor = vec4(0.0, 0.0, 1.0, 1.0);
    } else if (material.id == 2) {
        uFragColor = vec4(1.0, 0.0, 0.0, 1.0);
    } else {
        uFragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
}
