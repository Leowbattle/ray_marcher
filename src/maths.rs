use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
pub type real = f32;

pub use cgmath::InnerSpace;
pub use cgmath::Zero;

pub type Vec2 = cgmath::Vector2<real>;
pub type Vec3 = cgmath::Vector3<real>;

pub type Mat4 = cgmath::Matrix4<real>;

pub type Point3 = cgmath::Point3<real>;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Colour {
	pub r: real,
	pub g: real,
	pub b: real,
}

impl Colour {
	pub fn rgb(r: real, g: real, b: real) -> Colour {
		Colour { r, g, b }
	}
}

impl std::ops::Mul<real> for Colour {
	type Output = Colour;

	fn mul(self, x: real) -> Colour {
		Colour::rgb(self.r * x, self.g * x, self.b * x)
	}
}

pub fn real_mod(x: real, y: real) -> real {
	x - y * (x / y).floor()
}

pub fn vec3_mod(a: Vec3, b: Vec3) -> Vec3 {
	Vec3 {
		x: real_mod(a.x, b.x),
		y: real_mod(a.y, b.y),
		z: real_mod(a.z, b.z),
	}
}

pub fn vec3_abs(v: Vec3) -> Vec3 {
	Vec3::new(v.x.abs(), v.y.abs(), v.z.abs())
}

pub fn vec3_max(v: Vec3, x: real) -> Vec3 {
	Vec3::new(v.x.max(x), v.y.max(x), v.z.max(x))
}

pub fn vec3_reflect(i: Vec3, n: Vec3) -> Vec3 {
	i - 2.0 * n.dot(i) * n
}

pub fn real_one() -> real {
	1.0
}

pub fn vec3_ones() -> Vec3 {
	Vec3::new(1.0, 1.0, 1.0)
}
