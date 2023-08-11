#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Vector3 {
    #[field(1u32, "x", fixed32, singular)]
    pub x: f32,
    #[field(2u32, "y", fixed32, singular)]
    pub y: f32,
    #[field(3u32, "z", fixed32, singular)]
    pub z: f32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Triangle {
    #[field(1u32, "v0", nested, optional)]
    pub v0: Option<Box<Vector3>>,
    #[field(2u32, "v1", nested, optional)]
    pub v1: Option<Box<Vector3>>,
    #[field(3u32, "v2", nested, optional)]
    pub v2: Option<Box<Vector3>>,
    #[field(4u32, "normal", nested, optional)]
    pub normal: Option<Box<Vector3>>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Mesh {
    #[field(1u32, "triangles", nested, repeated)]
    pub triangles: Vec<Triangle>,
}
