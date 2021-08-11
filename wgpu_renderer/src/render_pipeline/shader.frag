#version 450

layout(set=1, binding=0)
uniform Camera {
    vec3 u_view_position;
    mat4 u_view_proj; // unused
};

layout(set = 2, binding = 0) uniform Light {
    vec3 light_position;
    vec3 light_color;
};

layout(location=1) in vec3 v_normal; 
layout(location=2) in vec3 v_position;

layout(location=0) out vec4 f_color;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 light_dir = normalize(light_position - v_position);

    // TODO: Use input object color instead of hard-coded color.
    vec4 object_color = vec4(1.0, 1.0, 1.0, 1.0);

    float ambient_strength = 0.05;
    vec3 ambient_color = light_color * ambient_strength;

    float diffuse_strength = max(dot(normal, light_dir), 0.0);
    vec3 diffuse_color = light_color * diffuse_strength;

    vec3 view_dir = normalize(u_view_position - v_position);
    vec3 reflect_dir = reflect(-light_dir, normal);
    float specular_strength = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular_color = specular_strength * light_color;

    vec3 result = (ambient_color + diffuse_color + specular_color) * object_color.xyz;
    f_color = vec4(result, object_color.a);
}