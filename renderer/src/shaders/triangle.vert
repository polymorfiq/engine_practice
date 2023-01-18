#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec4 pos;
layout (push_constant) uniform PushConstants {
    uint time;
} pcs;


layout (location = 0) out vec4 curr_color;
layout (location = 1) out vec4 next_color;
void main() {
    float color_transform = mod(pcs.time, 3000.0);

    vec4 color = vec4(1.0, 0.0, 0.0, 1.0);
    if(pos.x == -1.0) {
        color = vec4(0.0, 1.0, 0.0, 1.0);
    } else if(pos.x == 0.0) {
        color = vec4(0.0, 0.0, 1.0, 1.0);
    }

    if(color_transform < 1000.0) {
        curr_color = vec4(color.r, color.g, color.b, color.a);
        next_color = vec4(color.b, color.r, color.g, color.a);

    } else if(color_transform < 2000.0) {
        curr_color = vec4(color.b, color.r, color.g, color.a);
        next_color = vec4(color.g, color.b, color.r, color.a);
    } else {
        curr_color = vec4(color.g, color.b, color.r, color.a);
        next_color = vec4(color.r, color.g, color.b, color.a);
    }

    gl_Position = pos;
}
