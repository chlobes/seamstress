use crate::prelude::*;

#[repr(C)]
#[derive(Debug,Default,Copy,Clone)]
pub struct Vertex {
	pub pos: Vec3<f32>,
	pub color: [f32; 4],
	pub shine_color: [f32; 4],
	pub start_time: [f32; 3],
	pub shine_rate: f32,
	pub shine_bias: [f32; 3],
}

pub fn quadify(x: [Vertex; 4]) -> [Vertex; 6] {
	[x[0],x[1],x[3],x[0],x[3],x[2]]
}

pub fn quad(v: &mut Vec<Vertex>, pos: Vec2<f64>, z: usize, size: Vec2<f64>, color: [[f32; 4]; 4], shine_color: [[f32; 4]; 4], start_time: [[f64; 3]; 4], shine_rate: f64, shine_bias: [f64; 3]) {
	v.extend_from_slice(&make_quad(pos, z, size, color, shine_color, start_time, shine_rate, shine_bias));
}

pub fn make_quad(pos: Vec2<f64>, z: usize, size: Vec2<f64>, color: [[f32; 4]; 4], shine_color: [[f32; 4]; 4], start_time: [[f64; 3]; 4], shine_rate: f64, shine_bias: [f64; 3]) -> [Vertex; 6] {
	let offsets = [
		vec2(0.0,0.0),
		vec2(0.0,1.0),
		vec2(1.0,0.0),
		vec2(1.0,1.0),
	];
	let mut r = [Vertex::default(); 4];
	for i in 0..4 {
		r[i] = Vertex {
			pos: (pos + size * offsets[i]).extend(get_z(z)).f32(),
			color: color[i],
			shine_color: shine_color[i],
			start_time: [start_time[i][0] as f32, start_time[i][1] as f32, start_time[i][2] as f32],
			shine_rate: shine_rate as f32,
			shine_bias: [shine_bias[0] as f32, shine_bias[1] as f32, shine_bias[2] as f32],
		};
	}
	quadify(r)
}

pub fn get_z(z: usize) -> f64 { z as f64 / 1e4 }

/*#[derive(Debug)]
struct Builder {
	pos: Vec2<f32>,
	size: Vec2<f32>,
	z: u32,
	trans: Mat2<f32>,
	color: 
}

impl Builder {
	fn new() -> Self {
		Self {
			pos: Vec2::zero(),
			size: Vec2::one(),
			z: 0,
			texture: 0,
			trans: Mat2::ident(),
			uv_offset: Vec2::zero(),
			uv_size: Vec2::one(),
			fade: Vec2::zero(),
		}
	}
	
	fn pos(mut self, x: Vec2<f64>) -> Self { self.pos = x.f32(); self }
	fn size(mut self, x: Vec2<f64>) -> Self { self.size = x.f32(); self }
	fn z(mut self, x: u32) -> Self { self.z = x; self }
	fn texture(mut self, x: usize) -> Self { self.texture = x; self }
	fn trans(mut self, x: Mat2<f64>) -> Self { self.trans = x.f32(); self }
	#[allow(unused)]
	fn uv_offset(mut self, x: Vec2<f64>) -> Self { self.uv_offset = x.f32(); self }
	#[allow(unused)]
	fn uv_size(mut self, x: Vec2<f64>) -> Self { self.uv_size = x.f32(); self }
	fn fade(mut self, x: Vec2<f64>) -> Self { self.fade = x.f32(); self }
	
	fn quad(mut self, v: &mut Vec<Vertex>) {
		self.size /= 2.0;
		//self.size *= 1.1; //increase quad size slightly to account for wierd precision bugs
		let p = [
			self.pos - self.trans * self.size,
			self.pos + self.trans * vec2(self.size.x, -self.size.y),
			self.pos + self.trans * self.size,
			self.pos + self.trans * self.size,
			self.pos - self.trans * vec2(self.size.x, -self.size.y),
			self.pos - self.trans * self.size
		];
		let mut r = [Vertex::default(); 6];
		let (uvs, min, max) = quad_uvs(self.texture, self.uv_offset, self.uv_size);
		for i in 0..6 {
			r[i].uv = uvs[i]; r[i].uv_min = min; r[i].uv_max = max;
			r[i].pos = p[i].extend(self.z as f32 * 0.0001);
			r[i].size = self.size / (TEXTURE_SIZE.f32() * TEXTURE_RES as f32);
			r[i].fade = self.fade;
		}
		v.extend_from_slice(&r);
	}
}

fn quad_uvs(tex: usize, offset: Vec2<f32>, size: Vec2<f32>) -> ([Vec2<f32>; 6], Vec2<f32>, Vec2<f32>) {
	let size = size / TEXTURE_SIZE.f32();
	let coords = vec2(tex % TEXTURE_SIZE.x, tex / TEXTURE_SIZE.x);
	let mut start = (coords.f32() + offset) / TEXTURE_SIZE.f32();
	start.y = 1.0 - size.y - start.y;
	let end = start + size;
	let (s, e) = pixel_correct(start, end);
	([s, vec2(e.x,s.y), e, e, vec2(s.x,e.y), s], s, e)
}

fn pixel_correct(start: Vec2<f32>, end: Vec2<f32>) -> (Vec2<f32>, Vec2<f32>) {
	(start + 0.5 / TEXTURE_RES as f32, end - 0.5 / TEXTURE_RES as f32)
}*/
