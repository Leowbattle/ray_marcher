use crate::maths::*;
use crate::scene::Scene;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RenderError {
	BufferTooSmall,
}

impl fmt::Display for RenderError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Buffer was too small")
	}
}

impl Error for RenderError {}

fn get_direction(pos: Vec2, resolution: Vec2, fov: real) -> Vec3 {
	let xy = pos - resolution / 2.0;
	let z = resolution.y / fov.tan() / 2.0;
	Vec3::new(xy.x, xy.y, -z).normalize()
}

pub fn render(
	scene: &Scene,
	width: usize,
	height: usize,
	buffer: &mut [u8],
) -> Result<(), RenderError> {
	let stride = width * std::mem::size_of::<image::Rgb<u8>>();
	let required_size = stride * height;
	if buffer.len() < required_size {
		return Err(RenderError::BufferTooSmall);
	}

	let fov = scene.camera.fov.to_radians();
	let resolution = Vec2::new(width as real, height as real);

	let view_matrix = Mat4::look_at(
		Point3::new(
			scene.camera.position.x,
			scene.camera.position.y,
			scene.camera.position.z,
		),
		Point3::new(
			scene.camera.target.x,
			scene.camera.target.y,
			scene.camera.target.z,
		),
		Vec3::unit_y(),
	);

	for y in 0..height {
		for x in 0..width {
			let mut direction = get_direction(Vec2::new(x as real, y as real), resolution, fov);
			direction = -(view_matrix * direction.extend(0.0)).truncate();

			let result = scene
				.objects
				.iter()
				.filter_map(|o| {
					ray_march(
						|p| o.distance(p),
						scene.camera.position,
						direction,
						0.1,
						100.0,
						100,
					)
					.map(|d| (o, d))
				})
				.min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

			let colour = match result {
				Some((object, distance)) => {
					let p = scene.camera.position + direction * distance;
					let normal = get_normal(|p| object.distance(p), p);
					let colour = shade(normal, p, scene.light.position);
					// let colour = normal * 0.5 + Vec3::new(0.5, 0.5, 0.5);
					Colour::from([
						(colour.x * 255.0) as u8,
						(colour.y * 255.0) as u8,
						(colour.z * 255.0) as u8,
					])
				}
				None => Colour::from([0, 0, 0]),
			};

			let i = (y * width + x) * std::mem::size_of::<image::Rgb<u8>>();
			buffer[i + 0] = colour[0];
			buffer[i + 1] = colour[1];
			buffer[i + 2] = colour[2];
		}
	}

	Ok(())
}

const EPSILON: f32 = 1e-4;

pub fn ray_march(
	scene_sdf: impl Fn(Vec3) -> real,
	eye: Vec3,
	direction: Vec3,
	min_dist: real,
	max_dist: real,
	max_steps: usize,
) -> Option<real> {
	let mut depth = min_dist;
	for _ in 0..max_steps {
		let dist = scene_sdf(eye + depth * direction);
		if dist < EPSILON {
			return Some(depth);
		}
		depth += dist;
		if depth >= max_dist {
			return None;
		}
	}
	None
}

fn get_normal(scene_sdf: impl Fn(Vec3) -> real, p: Vec3) -> Vec3 {
	Vec3 {
		x: scene_sdf(Vec3::new(p.x + EPSILON, p.y, p.z))
			- scene_sdf(Vec3::new(p.x - EPSILON, p.y, p.z)),
		y: scene_sdf(Vec3::new(p.x, p.y + EPSILON, p.z))
			- scene_sdf(Vec3::new(p.x, p.y - EPSILON, p.z)),
		z: scene_sdf(Vec3::new(p.x, p.y, p.z + EPSILON))
			- scene_sdf(Vec3::new(p.x, p.y, p.z - EPSILON)),
	}
	.normalize()
}

fn shade(normal: Vec3, pixel_pos: Vec3, light_pos: Vec3) -> Vec3 {
	// Lambertian diffuse shading
	let light_dir = (light_pos - pixel_pos).normalize();
	let diffuse_strength = normal.dot(light_dir).max(0.0);
	Vec3::new(diffuse_strength, diffuse_strength, diffuse_strength)
}
