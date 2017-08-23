#version 330

in vec2 v_uv;

out vec4 f_color;

uniform sampler2D tex;

void main() {
    f_color = texture(tex, v_uv);
    
    if (f_color.a < 0.25) {
        gl_FragDepth = 0;
    } else {
        gl_FragDepth = gl_FragCoord.z;
    }
}
