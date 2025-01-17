use bevy::{app::App, asset::embedded_asset, reflect::Reflect, render::{render_resource::{Buffer, ShaderRef}, renderer::RenderDevice}};

use crate::{noise::ComputeNoise, render::pipeline::NoiseOp, shader::ComputeNoiseShader};

use super::ComputeNoiseModifier;

#[derive(Clone, Reflect, Default)]
pub struct Invert;

impl ComputeNoise for Invert {
    const NOISE_OP: NoiseOp = NoiseOp::Modifier; 

    fn buffers(&self, _render_device: &RenderDevice) -> Vec<Buffer> {
        Vec::new()
    }
}

impl ComputeNoiseModifier for Invert {
    fn embed_shaders(app: &mut App) {     
        embedded_asset!(app, "shaders/invert.wgsl");
    }

    fn shader() -> ShaderRef {
       "embedded://bevy_compute_noise/noise/modifiers/shaders/invert.wgsl".into() 
    }
}

impl ComputeNoiseShader for Invert {
    fn function_name() -> &'static str {
        "invert"
    }

    fn import_path() -> &'static str {
        "bevy_compute_noise::invert"
    }

    fn struct_name() -> Option<&'static str> {
        None
    }
}