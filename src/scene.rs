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
pub struct Light {
	pub position: Vec3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
	pub camera: Camera,
	pub light: Light,
	pub objects: Vec<Object>,
}

// impl Scene {
// 	pub fn empty() -> Scene {
// 		Scene {
// 			camera: Camera {
// 				pos: Vec3::zero(),
// 				target: Vec3::zero(),
// 				fov: 45.0,
// 			},
// 			light: Light { pos: Vec3::zero() },
// 			objects: vec![],
// 		}
// 	}
// }
