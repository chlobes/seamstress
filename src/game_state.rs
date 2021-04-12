use crate::prelude::*;
use crate::boiler_plate::InputEvent;

const GRAVITY: f64 = 0.007;
const DRAG: Vec2<f64> = Vec2{ x: 20.0, y: 2.0, };
const FRICTION: f64 = 0.5;

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
		let movement_x = self.player.movement[0] as u8 as f64 - self.player.movement[1] as u8 as f64;
		self.player.vel.x += movement_x * self.player.move_speed();
		self.player.centre_pos.x += self.player.vel.x;
		for platform in self.platforms.iter() {
			let p = self.player.collision_point_x(platform.vel());
			if let Some(correction) = platform.point_collides(p, self.player.vel()) {
				let diff = (self.player.vel() - platform.vel()).x;
				self.player.centre_pos.x -= diff.signum() * correction.x;
				self.player.vel.y -= self.player.vel.y.signum() * (diff.abs() * FRICTION).min(self.player.vel.y.abs());
				self.player.vel.x = platform.vel().x;
			}
		}
		self.player.vel.y -= GRAVITY;
		if self.player.grounded && self.player.movement[2] {
			self.player.vel.y += self.player.jump_vel();
			self.player.grounded = false;
		}
		self.player.centre_pos.y += self.player.vel.y;
		for platform in self.platforms.iter() {
			let p = self.player.collision_point_y(platform.vel());
			if let Some(correction) = platform.point_collides(p, self.player.vel()) {
				if self.player.vel.y.is_sign_negative() { //TODO: this will break if platforms move
					self.player.grounded = true;
				}
				let diff = (self.player.vel() - platform.vel()).y;
				self.player.centre_pos.y -= diff.signum() * correction.y;
				self.player.vel.x -= self.player.vel.x.signum() * (diff.abs() * FRICTION).min(self.player.vel.x.abs());
				self.player.vel.y = platform.vel().y;
			}
		}
		//let v = self.player.vel.magnitude();
		//self.player.vel *= 1.0 / (v * DRAG + 1.0);
		//do drag on both axes seperately because... it feels better? idk
		self.player.vel /= self.player.vel.abs() * DRAG + 1.0;
	}
	
	pub fn input_event(&mut self, e: InputEvent) {
		use InputEvent::*;
		match e {
			KeyDown(k) => match k.as_str() {
				"d" | "ArrowRight" => self.player.movement[0] = true,
				"a" | "ArrowLeft" => self.player.movement[1] = true,
				"w" | "ArrowUp" => self.player.movement[2] = true,
				_ => {},
			}
			KeyUp(k) => match k.as_str() {
				"d" | "ArrowRight" => self.player.movement[0] = false,
				"a" | "ArrowLeft" => self.player.movement[1] = false,
				"w" | "ArrowUp" => self.player.movement[2] = false,
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
	fn vel(&self) -> Vec2<f64>;
	
	fn centre_pos(&self) -> Vec2<f64> { self.pos() + self.size() / 2.0 }
	fn left_edge(&self) -> f64 { self.pos().x - self.size().x }
	fn right_edge(&self) -> f64 { self.pos().x + self.size().x }
	fn bottom_edge(&self) -> f64 { self.pos().y + self.size().y }
	fn top_edge(&self) -> f64 { self.pos().y - self.size().y }
	
	fn point_collides(&self, point: Vec2<f64>, velocity: Vec2<f64>) -> Option<Vec2<f64>> { //returns how much it collides by, if any
		let p = point;
		let v = velocity - self.vel();
		let a = self.pos(); let b = self.pos() + self.size();
		if a.x < p.x && b.x > p.x && a.y < p.y && b.y > p.y {
			Some(vec2(
				if v.x.is_sign_negative() { b.x - p.x } else { p.x - a.x },
				if v.y.is_sign_negative() { b.y - p.y } else { p.y - a.y },
			))
		} else {
			None
		}
	}
}

#[derive(Debug,Default,Copy,Clone)]
pub struct AABB {
	pub pos: Vec2<f64>,
	pub size: Vec2<f64>,
	pub vel: Vec2<f64>,
}

impl BoundingBox for AABB {
	fn pos(&self) -> Vec2<f64> { self.pos }
	fn size(&self) -> Vec2<f64> { self.size }
	fn vel(&self) -> Vec2<f64> { self.vel }
}

#[derive(Debug,Default)]
pub struct Player {
	pub centre_pos: Vec2<f64>,
	pub vel: Vec2<f64>,
	pub movement: [bool; 4],
	pub grounded: bool,
}

impl BoundingBox for Player {
	fn pos(&self) -> Vec2<f64> { self.centre_pos - self.size() / 2.0 }
	fn size(&self) -> Vec2<f64> { vec2(0.1, 0.2) }
	fn vel(&self) -> Vec2<f64> { self.vel }
}

impl Player {
	pub fn move_speed(&self) -> f64 { if self.grounded { 0.005 } else { 0.0025 } }
	pub fn jump_vel(&self) -> f64 { 0.13 }
	
	pub fn render(&self, v: &mut Vec<Vertex>) {
		quad(v, self.pos(), 10, self.size(), [[1.0,1.0,1.0,1.0]; 4], [vec2(1.0,0.0); 4]);
	}
	
	pub fn collision_point_x(&self, other_vel: Vec2<f64>) -> Vec2<f64> {
		vec2(self.centre_pos.x + self.size().x / 2.0 * (self.vel.x - other_vel.x).signum(),self.centre_pos.y)
	}
	
	pub fn collision_point_y(&self, other_vel: Vec2<f64>) -> Vec2<f64> {
		vec2(self.centre_pos.x,self.centre_pos.y + self.size().y / 2.0 * (self.vel.y - other_vel.y).signum())
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
	fn vel(&self) -> Vec2<f64> { Vec2::zero() }
}

impl Platform {
	pub fn render(&self, v: &mut Vec<Vertex>) {
		quad(v, self.pos(), 0, self.size(), [[0.0,0.0,0.0,1.0]; 4], [vec2(1.0,0.0); 4]);
	}
}
