pub use web_sys::WebGl2RenderingContext as GL;
pub use crate::vertex::*;
pub use std::rc::{Rc,Weak};
pub use std::cell::{Cell,RefCell,Ref,RefMut};
pub use crate::boiler_plate::*;
pub use crate::vertex::*;
pub use crate::game_state::*;

pub use math_lib::{vec2::*,vec3::*};

pub trait RcConv: Sized {
	fn rc(self) -> Rc<RefCell<Self>> { Rc::new(RefCell::new(self)) }
}
impl<T: Sized> RcConv for T {}

pub macro l() {
	&concat!(file!(), " ", line!())
}
