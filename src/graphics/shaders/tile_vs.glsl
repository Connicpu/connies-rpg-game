#version 330

// NOTE: This specialized shader always renders a 8x8 block of tiles

in vec2 pos;
in vec2 uv;

in uint tile_id;

uniform uint first_gid;
uniform vec3 world_base_pos;
uniform mat3 camera_view;
uniform mat4 camera_proj;

uniform sampler1D tileset;

out vec2 v_uv;
out uint v_tile_id;

void main() {
    int instX = gl_InstanceID & 7;
    int instY = -(gl_InstanceID / 8);

    vec2 vpos = pos * vec2(1.01, 1.01) + world_base_pos.xy + vec2(instX, instY);

    vec4 uv_rect = texelFetch(tileset, int(tile_id - first_gid), 0);

    gl_Position = vec4((vec3(vpos, 1.0) * camera_view).xy, world_base_pos.z, 1.0) * camera_proj;
    v_uv = mix(uv_rect.xy, uv_rect.zw, uv.xy);
    v_tile_id = tile_id;
}

