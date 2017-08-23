#version 330

// NOTE: This specialized shader always renders a 8x8 block of tiles

in vec2 pos;
in vec2 uv;

in uint tile_id;

uniform uint first_gid;
uniform uint end_gid;
uniform vec3 world_base_pos;

uniform mat4 proj;
uniform mat4 view;

uniform sampler1D tileset;

out vec2 v_uv;

void main() {
    int instX = gl_InstanceID & 7;
    int instY = -(gl_InstanceID / 8);
    vec2 vpos = pos * vec2(1.001, 1.001) + world_base_pos.xy + vec2(instX, instY);
    gl_Position = vec4(vpos, world_base_pos.z, 1.0) * view * proj;
    
    vec4 uv_rect = texelFetch(tileset, int(tile_id - first_gid), 0);

    if (tile_id >= first_gid && tile_id < end_gid) {
        v_uv = mix(uv_rect.xy, uv_rect.zw, uv.xy);
    } else {
        gl_Position.z = -1;
        v_uv = vec2(-1, -1);
    }
}

