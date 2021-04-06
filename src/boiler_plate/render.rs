use crate::prelude::*;
use web_sys::*;

pub fn compile_shader(
	gl: &GL,
	shader_type: u32,
	source: &str,
) -> Result<WebGlShader, String> {
	let shader = gl
		.create_shader(shader_type)
		.ok_or_else(|| String::from("Unable to create shader object"))?;
	gl.shader_source(&shader, source);
	gl.compile_shader(&shader);
	
	if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false)	{
		Ok(shader)
	} else {
		Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
	}
}

pub fn link_program(
	gl: &GL,
	vert_shader: &WebGlShader,
	frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
	let program = gl.create_program().ok_or_else(|| "Unable to create shader object".to_string())?;
	
	gl.attach_shader(&program, vert_shader); gl.attach_shader(&program, frag_shader); gl.link_program(&program);
	if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
		Ok(program)
	} else {
		Err(gl.get_program_info_log(&program).unwrap_or_else(|| "Unknown error creating program object".to_string()))
	}
}

const STRIDE: i32 = std::mem::size_of::<Vertex>() as i32;

pub fn setup_rendering() -> Result<(Rc<GL>, Rc<Vec<Option<WebGlUniformLocation>>>), JsValue> {
	let gl = Rc::new(canvas().get_context("webgl2")?.expect("browser does not support webgl").dyn_into::<GL>()?);
	
	let mut src = include_str!("shader.glsl").split("#![fragment_shader]\n");
	let vert_shader = compile_shader(
		&gl,
		GL::VERTEX_SHADER,
		src.next().unwrap(),
	)?;
	let frag_shader = compile_shader(
		&gl,
		GL::FRAGMENT_SHADER,
		src.next().unwrap(),
	)?;
	let program = link_program(&gl, &vert_shader, &frag_shader)?;
	gl.use_program(Some(&program));
	
	let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
	gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
	
	gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, STRIDE, 0);
	gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, STRIDE, 12);
	gl.vertex_attrib_pointer_with_i32(2, 1, GL::FLOAT, false, STRIDE, 28);
	gl.vertex_attrib_pointer_with_i32(3, 2, GL::FLOAT, false, STRIDE, 32);
	(0..4).for_each(|i| gl.enable_vertex_attrib_array(i));
	
	gl.viewport(0, 0, canvas().width() as i32, canvas().height() as i32);
	gl.clear_color(0.5, 0.5, 0.5, 1.0);
	gl.enable(GL::DEPTH_TEST);
	gl.depth_func(GL::GEQUAL);
	gl.enable(GL::BLEND);
	gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
	//gl.enable(GL::SAMPLE_COVERAGE);
	//gl.enable(GL::SAMPLE_ALPHA_TO_COVERAGE);
	
	let loc = Rc::new(vec![gl.get_uniform_location(&program, "time")]);
	
	Ok((gl, loc))
}

//TODO: split draw_arrays call and buffer_data calls into seperate functions so multiple buffers can be drawn?
pub fn render(gl: &GL, loc: &Rc<Vec<Option<WebGlUniformLocation>>>, verts: &[Vertex]) {
	let (u, mut u2) = (uniforms(), old_uniforms());
	if u != *u2 {
		gl.uniform1f(loc[0].as_ref(), u.time as f32);
		*u2 = u;
	}
	
	gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	gl.clear_depth(-1.0);
	
	let len = verts.len();
	unsafe {
		let ptr = verts.as_ptr() as *mut f32; //this transmute is safe because both Vertex and Vec2/3 are repr(C) so vertices are just a block of floats
		let len = len * (STRIDE as usize) / 4;
		std::mem::forget(verts);
		let verts = std::slice::from_raw_parts(ptr, len);
		let verts = js_sys::Float32Array::view(&verts);
		
		gl.buffer_data_with_array_buffer_view(
			GL::ARRAY_BUFFER,
			&verts,
			GL::STATIC_DRAW,
		)
	}
	
	gl.draw_arrays(GL::TRIANGLES, 0, len as i32);
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct UniformData {
	pub time: f64,
}

impl UniformData {
	pub fn add_time(mut self, x: f64) -> Self { self.time += x; set_uniforms(self); self }
}

impl Default for UniformData {
	fn default() -> Self {
		Self {
			time: 0.0,
		}
	}
}
