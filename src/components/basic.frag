precision mediump float;

uniform vec3 u_color;
uniform float u_time;

void main() {
    float r = sin(u_time * 0.003 * u_color.r);
    float g = sin(u_time * 0.005 * u_color.g);
    float b = sin(u_time * 0.007 * u_color.b);

    gl_FragColor = vec4(r, g, b, 1.0);
}