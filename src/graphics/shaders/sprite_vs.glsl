#version 330

in vec2 pos;
in vec2 uv;

in vec2 center;
in vec2 scale;
in mat2 rot;
in vec4 uv_rect;
in vec3 world_pos;

uniform mat3 camera_view;
uniform mat4 camera_proj;

out vec2 v_uv;

void main() {
    vec2 vpos = (pos + center) * scale * rot + world_pos.xy;

    gl_Position = vec4((vec3(vpos, 1.0) * camera_view).xy, world_pos.z, 1.0) * camera_proj;
    v_uv = mix(uv_rect.xy, uv_rect.zw, uv.xy);
}

