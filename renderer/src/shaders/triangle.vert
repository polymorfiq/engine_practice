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


mat4 gen_ortho_matrix(float left, float right, float top, float bottom, float near, float far) {
    return mat4(
        2.0/(right-left),    0.0,     0.0,     -(right+left)/(right-left),
        0.0,    2.0/(bottom-top),     0.0,     -(bottom+top)/(bottom-top),
        0.0,    0.0,    1.0/(near-far),    near/(near-far),
        0.0,  0.0,   0.0,     1.0
    );
}

mat4 gen_persp_matrix(float fov, float aspect, float near, float far) {
    return mat4(
        0.5,    0.0,     0.0,     0.0,
        0.0,    1/tan(fov/2.0),     0.0,     0.0,
        0.0,    0.0,    far/(far-near),    -near*(far-near),
        0.0,  0.0,   1.0,     0.0
    );
}

mat4 X = mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
);

mat4 look_at(vec3 eye, vec3 at, vec3 up) {
  vec3 zaxis = normalize(at - eye);    
  vec3 xaxis = normalize(cross(zaxis, up));
  vec3 yaxis = cross(xaxis, zaxis);

  zaxis = -1 * zaxis;

  mat4 viewMatrix = {
    vec4(xaxis.x, xaxis.y, xaxis.z, -dot(xaxis, eye)),
    vec4(yaxis.x, yaxis.y, yaxis.z, -dot(yaxis, eye)),
    vec4(zaxis.x, zaxis.y, zaxis.z, -dot(zaxis, eye)),
    vec4(0, 0, 0, 1)
  };

  return viewMatrix;
}

mat4 gen_view_matrix() {
    // the lookAt function requires three parameters:
    //  an eyepos - this is a vec3 variable where the camera is in world coords.
    // a target - this is the point your camera is looking at.
    // an Up vector - this specifies the direction for the top of your camera

    vec3 eyepos = vec3(0.0, 0.4, 1.0);   // camera slight above the origin.
    vec3 target = vec3(0.0, 0.4, -1.0);  // Camera looking straight down Z axis
    vec3 up = vec3(0.0, 1.0, 0.0); // the UP vector for the camera is simply straight up.

    return mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    
    // return look_at(eyepos, target, up);
}

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
    mat4 M = anim.start_transform + curr_anim_offset;

    mat4 V = gen_view_matrix();
    mat4 P = gen_persp_matrix(100.0 / 57.2958, 1000.0/1000.0, 1.0, -1.0);
    gl_Position = P * V * M * pos;

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