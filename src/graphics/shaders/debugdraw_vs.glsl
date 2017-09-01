#version 330

in vec2 pos;
in vec3 color;

out vec3 v_color;

uniform mat4 proj;
uniform mat4 view;

void main() {
    gl_Position = vec4(pos, 0, 1) * view * proj;
    v_color = color;
}
