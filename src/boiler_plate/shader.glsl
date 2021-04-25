#version 300 es

in highp vec3 pos;
in highp vec4 v_color;
in highp vec4 v_shine_color;
in highp float v_start_time;
in highp float v_shine_rate;

out highp vec4 color;
out highp vec4 shine_color;
out highp float start_time;
out highp float shine_rate;

void main() {
	color = v_color;
	shine_color = v_shine_color;
	start_time = v_start_time;
	shine_rate = v_shine_rate;
	gl_Position = vec4(pos,1.0);
}

#![fragment_shader]
#version 300 es

in highp vec4 color;
in highp vec4 shine_color;
in highp float start_time;
in highp float shine_rate;

uniform highp float time;

out mediump vec4 fragcolor;

void main() {
	highp float shine = pow(abs(sin((time - start_time) * shine_rate)),5.0) * shine_color.a;
	highp float alpha = color.a * (1.0 - shine); //if shine is strong, you can see less of underneath?
	highp float a = alpha + shine;
	fragcolor = vec4(color.rgb * alpha + shine_color.rgb * shine,a);
}
