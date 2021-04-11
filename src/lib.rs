#![feature(decl_macro,const_float_bits_conv)]
mod prelude; use prelude::*;
mod boiler_plate;
mod vertex;
mod game_state;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	initial_setup();
	setup_input_events();
	let (gl, u_loc) = setup_rendering().expect(l!());
	
	let mut game_state = GameState::new();
	game_state.platforms.push(Platform {
		centre_pos: vec2(0.0,-0.8),
		size: vec2(2.0,0.4),
	});
	
	let f = None.rc(); let g = f.clone();
	*f.borrow_mut() = Some(Closure::wrap(Box::new(move|| {
		uniforms().add_time(1.0);
		//run_callbacks();
		game_state.tick();
		for event in input_events().drain(..) {
			game_state.input_event(event);
		}
		render(&gl, &u_loc, &game_state.render());
		request_animation_frame(g.borrow().as_ref().unwrap());
	}) as Box<dyn FnMut()>));
	request_animation_frame(f.borrow().as_ref().unwrap());
	Ok(())
}
