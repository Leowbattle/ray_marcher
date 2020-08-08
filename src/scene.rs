use serde::{Deserialize, Serialize};

use crate::maths::*;
use crate::object::Object;

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
	pub position: Vec3,
	pub target: Vec3,
	pub fov: real,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Environment {
	pub ambient_light: real,
}

impl Default for Environment {
	fn default() -> Environment {
		Environment { ambient_light: 0.0 }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Light {
	pub position: Vec3,

	#[serde(default = "real_one")]
	pub strength: real,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
	pub camera: Camera,
	#[serde(default)]
	pub environment: Environment,
	pub light: Light,
	pub objects: Vec<Object>,
}
