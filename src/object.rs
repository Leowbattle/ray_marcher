///! Reference: https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
use serde::{Deserialize, Serialize};

use crate::maths::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
	#[serde(default = "Vec3::zero")]
	pub position: Vec3,

	#[serde(default)]
	pub material: Material,
	pub geometry: Box<dyn Geometry>,
}

impl Object {
	pub fn distance(&self, p: Vec3) -> real {
		self.geometry.distance(p - self.position)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
	pub colour: Colour,
}

impl Default for Material {
	fn default() -> Material {
		Material {
			colour: Colour::rgb(1.0, 0.0, 0.0),
		}
	}
}

#[typetag::serde(tag = "type")]
pub trait Geometry: std::fmt::Debug {
	/// The distance from the point p to the surface of the object.
	fn distance(&self, p: Vec3) -> real;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sphere {
	#[serde(default = "real_one")]
	pub radius: real,
}

#[typetag::serde]
impl Geometry for Sphere {
	fn distance(&self, p: Vec3) -> real {
		p.magnitude() - self.radius
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfiniteRepetition {
	#[serde(default = "default_period")]
	pub period: Vec3,
	pub child: Box<dyn Geometry>,
}

#[typetag::serde]
impl Geometry for InfiniteRepetition {
	fn distance(&self, p: Vec3) -> real {
		self.child
			.distance(vec3_mod(p + 0.5 * self.period, self.period) - 0.5 * self.period)
	}
}

fn default_period() -> Vec3 {
	Vec3::new(1.0, 1.0, 1.0)
}
