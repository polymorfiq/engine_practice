#version 400
#extension GL_ARB_separate_shader_objects : enable
#extension GL_ARB_shading_language_420pack : enable

layout (location = 0) in vec3 pos;

layout(binding = 0) uniform Transformation {
    mat4 matrix;
} transformation;

layout (push_constant) uniform PushConstants {
    uint time;
} pcs;

mat4 PerspectiveMatrix(float l, float r, float t, float b, float n, float f) {
    return transpose(mat4(
        (2.0*n)/(r-l), 0.0, (r+l)/(r-l), 0.0,
        0.0, (2.0*n)/(b-t), (b+t)/(b-t), 0.0,
        0.0, 0.0, n/(f-n), (n*f)/(f-n),
        0.0, 0.0, -n, 1.0
    ));
}

mat4 P = PerspectiveMatrix(-1, 1, -1, 1, 5, 0);

mat4 V = transpose(mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
));

// layout (location = 0) out vec4 curr_color;
// layout (location = 1) out vec4 next_color;
void main() {
    mat4 M = transformation.matrix;
    gl_Position = P * V * M * vec4(pos, 1.0);
}