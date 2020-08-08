#[allow(non_camel_case_types)]
pub type real = f32;

pub use cgmath::InnerSpace;
pub use cgmath::Zero;

pub type Vec2 = cgmath::Vector2<real>;
pub type Vec3 = cgmath::Vector3<real>;

pub type Mat4 = cgmath::Matrix4<real>;

pub type Point3 = cgmath::Point3<real>;

pub type Colour = image::Rgb<u8>;
