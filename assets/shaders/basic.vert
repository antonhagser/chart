precision mediump float;

attribute vec2 a_position;
attribute vec3 a_color;
attribute float a_extra;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_proj;

varying vec4 v_color;

void main() {
    vec4 pos = u_model * vec4(a_position, 0.0, 1.0);
    vec4 view_pos = u_view * pos;

    gl_Position = u_proj * view_pos;
    v_color = vec4(a_color.rgb, 1.0);
}