#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec4 pos;

layout(binding = 0) uniform UniformBufferObject {
    mat4 start_transform;
    mat4 end_transform;
    uint start_time;
    uint end_time;
} anim;

layout (push_constant) uniform PushConstants {
    uint time;
} pcs;


layout (location = 0) out vec4 curr_color;
layout (location = 1) out vec4 next_color;
void main() {

    // Calculate position of vertex, in animation
    uint total_duration = anim.end_time - anim.start_time;
    uint curr_duration = pcs.time - anim.start_time;
    float anim_percentage = max(min(1.0, float(curr_duration) / float(total_duration)), 0.0);
    if (total_duration == 0) {
        anim_percentage = 1.0;
    }
    
    mat4 end_anim_offset = anim.end_transform - anim.start_transform;
    mat4 curr_anim_offset = (anim_percentage * end_anim_offset);
    mat4 curr_transform = anim.start_transform + curr_anim_offset;
    // float wc = pos[2];
    // gl_Position = (curr_transform * pos) / wc;
    // mat4 curr_transform = mat4(
    //     1.0, 0.0, 0.0, 0.0,
    //     0.0, 1.0, 0.0, 0.0,
    //     0.0, 0.0, 1.0, 0.0,
    //     0.0, 0.0, 0.0, 1.0
    // );
    gl_Position = curr_transform * pos;

    // Calculate color of vertex, in animation based on time
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
}
