use bevy::{app::App, render::render_resource::ShaderRef};

use crate::noise::ComputeNoise;

pub mod invert;

pub use invert::Invert;

pub trait ComputeNoiseModifier: ComputeNoise {
    fn embed_shaders(app: &mut App);
    fn shader() -> ShaderRef;
}