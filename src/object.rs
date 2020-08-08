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

// Shapes

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
pub struct Cube {
	#[serde(default = "vec3_ones")]
	pub size: Vec3,
}

#[typetag::serde]
impl Geometry for Cube {
	fn distance(&self, p: Vec3) -> real {
		let q = vec3_abs(p) - self.size;
		vec3_max(q, 0.0).magnitude() + q.x.max(q.y.max(q.z)).min(0.0)
	}
}

// Operators

#[derive(Debug, Serialize, Deserialize)]
pub struct InfiniteRepetition {
	#[serde(default = "default_period")]
	pub period: Vec3,
	pub child: Object,
}

#[typetag::serde]
impl Geometry for InfiniteRepetition {
	fn distance(&self, p: Vec3) -> real {
		self.child
			.distance(vec3_mod(p + 0.5 * self.period, self.period) - 0.5 * self.period)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Union {
	pub a: Object,
	pub b: Object,
}

#[typetag::serde]
impl Geometry for Union {
	fn distance(&self, p: Vec3) -> real {
		self.a.distance(p).min(self.b.distance(p))
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtraction {
	pub a: Object,
	pub b: Object,
}

#[typetag::serde]
impl Geometry for Subtraction {
	fn distance(&self, p: Vec3) -> real {
		(-self.b.distance(p)).max(self.a.distance(p))
	}
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Intersection {
	pub a: Object,
	pub b: Object,
}

#[typetag::serde]
impl Geometry for Intersection {
	fn distance(&self, p: Vec3) -> real {
		(self.b.distance(p)).max(self.a.distance(p))
	}
}

fn default_period() -> Vec3 {
	Vec3::new(1.0, 1.0, 1.0)
}
