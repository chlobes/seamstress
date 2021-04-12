use crate::prelude::*;
pub use wasm_bindgen::{prelude::*,JsCast};
use web_sys::*;

mod render; pub use render::*;

pub macro log( $( $t:tt )* ) {{
	let s = format!( $( $t )* );
	console::log_1(&s.clone().into());
	//tmp_logs().push(s);
}}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	window()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("should register `requestAnimationFrame` OK");
}

pub fn window() -> Window {
	web_sys::window().expect("no global `window` exists")
}

pub fn document() -> Document {
	window().document().expect(l!())
}

pub fn canvas() -> HtmlCanvasElement {
	document().get_element_by_id("canvas").expect(l!()).dyn_into().expect(l!())
}

fn wrap<F: 'static + Fn(A) -> O, A: 'static + wasm_bindgen::convert::FromWasmAbi, O: 'static + wasm_bindgen::convert::IntoWasmAbi>(f: F)
	-> Closure<dyn Fn(A) -> O> {
	Closure::wrap(Box::new(f))
}

pub macro bind($target: expr, $target2: expr, $f: expr) {
	let f = wrap($f);
	$target.add_event_listener_with_callback($target2, f.as_ref().unchecked_ref())
		.expect(&format!("failed to add function to html object: {}, with listener: {}",stringify!($target),$target2));
	f.forget();
}

pub fn setup_input_events() {
	bind!(canvas(), "wheel", move|e: WheelEvent| {
		input_events().push(InputEvent::Wheel(e.delta_y().signum()));
		false
	});
	bind!(canvas(), "click", move|_: MouseEvent| {
		input_events().push(InputEvent::LeftClick);
	});
	bind!(canvas(), "contextmenu", move|_: MouseEvent| {
		input_events().push(InputEvent::RightClick);
	});
	bind!(document(), "mousemove", move|e: MouseEvent| {
		set_mouse_coords(vec2(e.client_x(), e.client_y()));
	});
	bind!(document(), "keydown", move|e: KeyboardEvent| {
		input_events().push(InputEvent::KeyDown(e.key()));
	});
	bind!(document(), "keyup", move|e: KeyboardEvent| {
		input_events().push(InputEvent::KeyUp(e.key()));
	});
}

#[derive(Debug,Clone)]
pub enum InputEvent {
	Wheel(f64),
	LeftClick,
	RightClick,
	KeyDown(String),
	KeyUp(String),
}

use std::sync::atomic::{Ordering::Relaxed,AtomicU64};
pub fn initial_setup() {
	console_error_panic_hook::set_once();
	unsafe {
		macro init($($name: ident,)*) { $($name = Some(RefCell::default());)* }
		init!(INPUT_EVENTS,UNIFORMS,OLD_UNIFORMS,);
	}
}

static mut INPUT_EVENTS: Option<RefCell<Vec<InputEvent>>> = None;
pub fn input_events() -> RefMut<'static, Vec<InputEvent>> { unsafe { INPUT_EVENTS.as_ref().unwrap().borrow_mut() } }

static mut UNIFORMS: Option<RefCell<UniformData>> = None;
pub fn uniforms() -> UniformData { unsafe { UNIFORMS.as_ref().unwrap().borrow().clone() } }
pub fn set_uniforms(x: UniformData) { unsafe { *UNIFORMS.as_ref().unwrap().borrow_mut() = x; } }
static mut OLD_UNIFORMS: Option<RefCell<UniformData>> = None;
pub fn old_uniforms() -> RefMut<'static, UniformData> { unsafe { OLD_UNIFORMS.as_ref().unwrap().borrow_mut() } }
//pub fn time() -> f64 { uniforms().time }

/*static ID: AtomicU64 = AtomicU64::new(0);
pub fn new_id() -> u64 { ID.fetch_add(1, Relaxed) }*/

static SCREEN_COORDS: (AtomicU64, AtomicU64) = (AtomicU64::new(f64::NAN.to_bits()), AtomicU64::new(f64::NAN.to_bits()));
pub fn set_mouse_coords(c: Vec2<i32>) {
	let c = c.f64();
	let rect = canvas().get_bounding_client_rect();
	if c.x > rect.left() && rect.right() > c.x && c.y > rect.top() && rect.bottom() > c.y {
		let c = (c - vec2(rect.left(),rect.top())) / vec2(rect.width(),rect.height()) * 2.0 - 1.0;
		let c = vec2(c.x,-c.y).f64();
		SCREEN_COORDS.0.store(c.x.to_bits(), Relaxed); SCREEN_COORDS.1.store(c.y.to_bits(), Relaxed);
	} else {
		SCREEN_COORDS.0.store(f64::NAN.to_bits(), Relaxed); SCREEN_COORDS.1.store(f64::NAN.to_bits(), Relaxed);
	}
}
/*pub fn mouse_screen_coords() -> Option<Vec2<f64>> {
	let c = vec2(f64::from_bits(SCREEN_COORDS.0.load(Relaxed)), f64::from_bits(SCREEN_COORDS.1.load(Relaxed)));
	if c.is_nan().or() { None } else { Some(c) }
}*/
