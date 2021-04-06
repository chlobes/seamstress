#version 300 es

in highp vec3 pos;
in highp vec4 v_color;
in highp float v_start_time;
in highp vec2 v_blend;

out highp vec4 color;
out highp float start_time;
out highp vec2 blend;

uniform vec3 cam_pos;

void main() {
	
	gl_Position = vec4(pos,1.0);
}

#![fragment_shader]
#version 300 es

in highp vec4 color;
in highp float start_time;
in highp vec2 blend; //first value is how much of color variable to use, second variable is how much shine to use

uniform highp float time;

out mediump vec4 fragcolor;

void main() {
	highp float t = time - start_time;
	highp float s = 20.0; //shine change rate
	fragcolor = blend.x * color + blend.y * color.a * vec4(mod(t + 15.0, s) / s,mod(t + 10.0, s) / s,mod(t + 5.0, s) / s,0.0);
}
