use crate::maths::*;
use crate::object::*;
use crate::scene::*;

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
	let z = resolution.y / (fov / 2.0).tan();
	Vec3::new(xy.x, xy.y, -z).normalize()
}

fn get_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
	let f = (center - eye).normalize();
	let s = f.cross(up).normalize();
	let u = s.cross(f);

	Mat4::new(
		s.x, s.y, s.z, 0.0, u.x, u.y, u.z, 0.0, -f.x, -f.y, -f.z, 0.0, 0.0, 0.0, 0.0, 1.0,
	)
}

const MIN_DISTANCE: real = 0.1;
const MAX_DISTANCE: real = 100.0;
const MAX_STEPS: usize = 100;

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

	let view_matrix = get_view_matrix(scene.camera.position, scene.camera.target, -Vec3::unit_y());

	for y in 0..height {
		for x in 0..width {
			let mut direction = get_direction(Vec2::new(x as real, y as real), resolution, fov);
			direction = (view_matrix * direction.extend(0.0)).truncate();

			let result = scene
				.objects
				.iter()
				.filter_map(|o| {
					ray_march(|p| o.distance(p), scene.camera.position, direction).map(|d| (o, d))
				})
				.min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

			let colour = match result {
				Some((object, distance)) => {
					let p = scene.camera.position + direction * distance;
					let normal = get_normal(|p| object.distance(p), p);
					let colour =
						shade(&scene, normal, p, &object, &scene.environment, &scene.light);
					colour
				}
				None => scene.environment.background_colour,
			};

			let i = (y * width + x) * std::mem::size_of::<image::Rgb<u8>>();
			buffer[i + 0] = (colour.r * 255.0) as u8;
			buffer[i + 1] = (colour.g * 255.0) as u8;
			buffer[i + 2] = (colour.b * 255.0) as u8;
		}
	}

	Ok(())
}

const EPSILON: real = 1e-4;

pub fn ray_march(scene_sdf: impl Fn(Vec3) -> real, eye: Vec3, direction: Vec3) -> Option<real> {
	let mut depth = MIN_DISTANCE;
	for _ in 0..MAX_STEPS {
		let dist = scene_sdf(eye + depth * direction);
		if dist < EPSILON {
			return Some(depth);
		}
		depth += dist;
		if depth >= MAX_DISTANCE {
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

fn shade(
	scene: &Scene,
	normal: Vec3,
	pixel_pos: Vec3,
	object: &Object,
	environment: &Environment,
	light: &Light,
) -> Colour {
	let shadow = if in_shadow(scene, pixel_pos, light) {
		0.8
	} else {
		1.0
	};

	let mut light_dir = light.position - pixel_pos;
	let distance = light_dir.magnitude();
	light_dir = light_dir.normalize();
	let attenuation = 1.0 / (1.0 + distance * distance);
	let diffuse_strength =
		(normal.dot(light_dir).max(0.0) * attenuation * light.strength).clamp(0.0, 1.0);

	let light = environment.ambient_light + diffuse_strength;

	object.material.colour * light * shadow
}

fn in_shadow(scene: &Scene, p: Vec3, light: &Light) -> bool {
	let ray_dir = (light.position - p).normalize();

	match ray_march(
		|p| {
			scene
				.objects
				.iter()
				.map(|o| o.distance(p))
				.min_by(|a, b| a.partial_cmp(&b).unwrap())
				.unwrap()
		},
		p,
		ray_dir,
	) {
		Some(_) => true,
		None => false,
	}
}
