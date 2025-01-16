use bevy::{prelude::*, render::render_resource::{ShaderDefVal, ShaderRef}};

pub mod worley;
pub mod perlin;
pub mod fbm;

use bytemuck::Pod;
pub use worley::{Worley, WorleyFlags};
pub use perlin::{Perlin, PerlinFlags};
pub use fbm::Fbm;

use super::ComputeNoise;

pub trait ComputeNoiseGenerator: ComputeNoise + Pod {
    fn embed_shaders(app: &mut App);
    fn shader_2d() -> ShaderRef;
    fn shader_3d() -> ShaderRef;
    fn shader_def() -> ShaderDefVal;
}