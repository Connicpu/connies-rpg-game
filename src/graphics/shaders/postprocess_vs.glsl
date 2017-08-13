#version 330

in vec2 pos;
in vec2 uv;

out vec2 v_uv;

void main()
{
    v_uv = uv;
    gl_Position = vec4((pos - vec2(0.5, 0.5) * 2), 0, 1);
}
