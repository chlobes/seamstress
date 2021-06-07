#version 300 es

in highp vec3 pos;
in highp vec4 v_color;
in highp vec4 v_shine_color;
in highp vec3 v_start_time;
in highp float v_shine_rate;

out highp vec4 color;
out highp vec4 shine_color;
out highp vec3 start_time;
out highp float shine_rate;

uniform highp vec2 cam_pos;

void main() {
	color = v_color;
	shine_color = v_shine_color;
	start_time = v_start_time;
	shine_rate = v_shine_rate;
	gl_Position = vec4(pos.xy-cam_pos,pos.z,1.0);
}

#![fragment_shader]
#version 300 es

in highp vec4 color;
in highp vec4 shine_color;
in highp vec3 start_time;
in highp float shine_rate;

uniform highp float time;

out mediump vec4 fragcolor;

void main() {
	highp vec3 shine = abs(sin((time - start_time) * shine_rate));
	shine = vec3(pow(shine.x,5.0),pow(shine.y,5.0),pow(shine.z,5.0)) * shine_color.a;
	highp float alpha = color.a * (1.0 - (shine.r+shine.g+shine.b)/3.0); //if shine is strong, you can see less of underneath?
	highp vec3 a = alpha + shine;
	fragcolor = vec4(color.rgb * alpha + shine_color.rgb * shine,max(a.x,max(a.y,a.z)));
}
