#version 330

in vec2 pos;
in vec2 uv;

in vec2 center;
in vec2 scale;
in mat2 rotation;
in vec4 uv_rect;
in vec3 world_position;

out vec2 v_uv;

void main() {
    vec2 pos = (pos - center) * scale * rotation + world_position.xy;

    gl_Position = vec4(pos, world_position.z, 1.0);
    v_uv.x = mix(uv_rect.x, uv_rect.z, uv.x);
    v_uv.y = mix(uv_rect.y, uv_rect.w, uv.y);
}
