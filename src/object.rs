use serde::{Deserialize, Serialize};

use crate::maths::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
	#[serde(default = "Vec3::zero")]
	pub position: Vec3,

	pub geometry: Box<dyn Geometry>,
}

impl Object {
	pub fn distance(&self, p: Vec3) -> real {
		self.geometry.distance(p - self.position)
	}
}

#[typetag::serde(tag = "type")]
pub trait Geometry: std::fmt::Debug {
	/// The distance from the point p to the surface of the object.
	fn distance(&self, p: Vec3) -> real;
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
	pub r: real,
}

#[typetag::serde]
impl Geometry for Sphere {
	fn distance(&self, p: Vec3) -> real {
		p.magnitude() - self.r
	}
}
