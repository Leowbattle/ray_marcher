#![forbid(unsafe_code)]
#![feature(clamp)]

mod maths;
mod object;
mod renderer;
mod scene;

use scene::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	use clap::{App, Arg};

	let args = App::new("Ray Marcher")
		.arg(Arg::with_name("scene").required(true))
		.arg(Arg::with_name("output").default_value("out.png"))
		.arg(Arg::with_name("width"))
		.arg(Arg::with_name("height"))
		.get_matches();

	let scene_path = args.value_of("scene").unwrap();
	let output_path = args.value_of("output").unwrap();
	let width = match args.value_of("width") {
		Some(s) => s.parse::<usize>()?,
		None => 640,
	};
	let height = match args.value_of("height") {
		Some(s) => s.parse::<usize>()?,
		None => 360,
	};
	let scene_data = std::fs::read_to_string(scene_path)?;
	let scene: Scene = serde_json::from_str(&scene_data)?;

	let mut buffer = vec![0; width * height * std::mem::size_of::<image::Rgb<u8>>()];
	renderer::render(&scene, width, height, buffer.as_mut())?;

	// let perf = time(|| {
	// 	renderer::render(&scene, width, height, buffer.as_mut());
	// });
	// dbg!(perf);

	image::save_buffer(
		output_path,
		&buffer,
		width as u32,
		height as u32,
		image::ColorType::Rgb8,
	)?;

	Ok(())
}

use std::time::{Duration, Instant};

#[allow(dead_code)]
fn time(mut f: impl FnMut()) -> Duration {
	let start = Instant::now();
	const TIMES: u32 = 100;
	for _ in 0..TIMES {
		f();
	}
	let end = Instant::now();
	(end - start) / TIMES
}
