#version 330

in vec2 v_uv;
in uint v_tile_id;

out vec4 f_color;

uniform sampler2D tex;

void main() {
    if (v_tile_id != 0) {
        f_color = texture(tex, v_uv);
    } else {
        f_color = vec4(0.0, 0.0, 0.0, 0.0);
    }

    if (f_color.a < 0.25) {
        gl_FragDepth = 0;
    } else {
        gl_FragDepth = gl_FragCoord.z;
    }
}
