#version 450

layout(set=0, binding=0)
uniform ModelTransformation {
    mat4 u_model_transf;
};

layout(set=1, binding=0)
uniform Camera {
    vec3 u_view_position; // unused
    mat4 u_view_proj;
};

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;

layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_position;

void main() {
    // TODO: This matrix math should be pulled out of the shader.
    mat3 normal_matrix = mat3(transpose(inverse(u_model_transf)));
    v_normal = normal_matrix * a_normal;

    v_position = a_position;

    gl_Position = u_view_proj * u_model_transf * vec4(a_position, 1.0);
}