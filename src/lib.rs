#![feature(decl_macro,const_float_bits_conv)]
mod prelude; use prelude::*;
mod boiler_plate;
mod vertex;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	initial_setup();
	setup_input_events();
	let (gl, u_loc) = setup_rendering().expect(l!());
	
	let f = None.rc(); let g = f.clone();
	*f.borrow_mut() = Some(Closure::wrap(Box::new(move|| {
		uniforms().add_time(1.0);
		//run_callbacks();
		for event in input_events().drain(..) {
			
		}
		let mut verts: Vec<Vertex> = Vec::new();
		render(&gl, &u_loc, &verts);
		request_animation_frame(g.borrow().as_ref().unwrap());
	}) as Box<dyn FnMut()>));
	request_animation_frame(f.borrow().as_ref().unwrap());
	Ok(())
}
