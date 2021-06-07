pub use web_sys::WebGl2RenderingContext as GL;
pub use crate::vertex::*;
pub use std::rc::{Rc,Weak};
pub use std::cell::{Cell,RefCell,Ref,RefMut};
pub use crate::boiler_plate::*;
pub use crate::vertex::*;
pub use crate::game_state::*;

pub use math_lib::{vec2::*,vec3::*,traits::*};
pub use array_tuple::ArrayTuple;

pub trait RcConv: Sized {
	fn rc(self) -> Rc<RefCell<Self>> { Rc::new(RefCell::new(self)) }
}
impl<T: Sized> RcConv for T {}

pub macro l() {
	&concat!(file!(), " ", line!())
}

/*pub fn randf() -> f64 {
	randi() as f64 / u64::max_value() as f64
}

pub fn randi() -> u64 {
	let mut x = [0; 8];
	window().crypto().expect(l!()).get_random_values_with_u8_array(&mut x).expect(l!());
	log!("{:?}",x);
	unsafe { std::mem::transmute(x) }
}*/

#[derive(Debug,Copy,Clone)]
pub struct RandGen { //seeded random generator
	state: [u64; 5],
}

impl RandGen {
	pub fn new(seed: u64) -> Self {
		let seed = (u64::max_value() / 7 * 4).wrapping_add(seed); //small seeds cause the first few values to be wierd
		Self {
			state: [seed, seed % 865941, seed % 45129, seed % 963, seed % 7437],
		}
	}
	
	pub fn skip(&mut self, n: usize) -> Self {
		for _ in 0..n {
			self.next();
		}
		*self
	}
	
	pub fn next(&mut self) -> u64 {
		let mut t = self.state[3];
		t ^= t >> 2;
		t ^= t << 1;
		self.state[3] = self.state[2]; self.state[2] = self.state[1]; self.state[1] = self.state[0];
		let s = self.state[0];
		t ^= s;
		t ^= s << 4;
		self.state[0] = t;
		self.state[4] += 362437;
		t + self.state[4]
	}
	
	pub fn f64(&mut self) -> f64 {
		self.next() as f64 / u64::max_value() as f64
	}
}

