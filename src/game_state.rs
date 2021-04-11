use crate::prelude::*;
use crate::boiler_plate::InputEvent;

const GRAVITY: f64 = 0.01;

#[derive(Debug)]
pub struct GameState {
	pub player: Player,
	pub platforms: Vec<Platform>,
}

impl GameState {
	pub fn new() -> Self {
		Self {
			player: Default::default(),
			platforms: Vec::new(),
		}
	}
	
	pub fn tick(&mut self) {
		let old_pos = self.player.centre_pos;
		
		let movement_x = self.player.movement[0] as u8 as f64 - self.player.movement[1] as u8 as f64;
		self.player.centre_pos.x += movement_x * self.player.move_speed();
	}
	
	pub fn input_event(&mut self, e: InputEvent) {
		use InputEvent::*;
		match e {
			KeyDown(k) => match k.as_str() {
				"d" => self.player.movement[0] = true,
				"a" => self.player.movement[1] = true,
				"w" => self.player.movement[2] = true,
				_ => log!("{}",k),
			}
			KeyUp(k) => match k.as_str() {
				"d" => self.player.movement[0] = false,
				"a" => self.player.movement[1] = false,
				"w" => self.player.movement[2] = false,
				_ => {},
			}
			_ => {},
		}
	}
	
	pub fn render(&self) -> Vec<Vertex> {
		let mut r = Vec::new(); let v = &mut r; 
		self.player.render(v);
		self.platforms.iter().for_each(|x| x.render(v));
		r
	}
}

pub trait BoundingBox {
	fn pos(&self) -> Vec2<f64>;
	fn size(&self) -> Vec2<f64>;
	
	fn centre_pos(&self) -> Vec2<f64> { self.pos() + self.size() / 2.0 }
	fn left_edge(&self) -> f64 { self.pos().x - self.size().x }
	fn right_edge(&self) -> f64 { self.pos().x + self.size().x }
	fn bottom_edge(&self) -> f64 { self.pos().y + self.size().y }
	fn top_edge(&self) -> f64 { self.pos().y - self.size().y }
}

#[derive(Debug,Default)]
pub struct Player {
	pub centre_pos: Vec2<f64>,
	pub vel: Vec2<f64>,
	pub movement: [bool; 4],
}

impl BoundingBox for Player {
	fn pos(&self) -> Vec2<f64> { self.centre_pos - self.size() / 2.0 }
	fn size(&self) -> Vec2<f64> { vec2(0.1, 0.2) }
}

impl Player {
	pub fn move_speed(&self) -> f64 { 0.001 }
	
	pub fn render(&self, v: &mut Vec<Vertex>) {
		quad(v, self.pos(), 10, self.size(), [[1.0,1.0,1.0,1.0]; 4], [vec2(1.0,0.0); 4]);
	}
}

#[derive(Debug,Default)]
pub struct Platform {
	pub centre_pos: Vec2<f64>,
	pub size: Vec2<f64>,
}

impl BoundingBox for Platform {
	fn pos(&self) -> Vec2<f64> { self.centre_pos - self.size() / 2.0 }
	fn size(&self) -> Vec2<f64> { self.size }
}

impl Platform {
	pub fn render(&self, v: &mut Vec<Vertex>) {
		quad(v, self.pos(), 0, self.size(), [[0.0,0.0,0.0,1.0]; 4], [vec2(1.0,0.0); 4]);
	}
}
