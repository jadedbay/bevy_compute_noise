use bevy::{asset::embedded_asset, prelude::*, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, ShaderDefVal, ShaderRef}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};

use crate::{render::pipeline::NoiseOp, shader::ComputeNoiseShader};

use super::{ComputeNoise, ComputeNoiseGenerator};

#[derive(Clone, Copy, Reflect, PartialEq, Debug, Pod, Zeroable)]
#[reflect(Default)]
#[repr(C)]
pub struct Worley {
    pub seed: u32,
    pub frequency: f32,
    pub flags: u32,
}

bitflags::bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WorleyFlags: u32 {
        const TILEABLE = 1 << 0;
    }
}

impl Default for Worley {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 5.0,
            flags: 0,
        }
    }
}

impl ComputeNoise for Worley {
    const NOISE_OP: NoiseOp = NoiseOp::Generator; 

    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("worley2d_points_buffer"),
                    contents: &bytemuck::cast_slice(&[self.clone()]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            ),
        ] 
    }
}

impl ComputeNoiseGenerator for Worley {
    fn shader_2d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/generators/shaders/worley_2d.wgsl".into()
    }

    fn shader_3d() -> ShaderRef {
        "embedded://bevy_compute_noise/noise/generators/shaders/worley_3d.wgsl".into()
    }

    fn embed_shaders(app: &mut App) {
        embedded_asset!(app, "shaders/worley_2d.wgsl");
        embedded_asset!(app, "shaders/worley_3d.wgsl");
    }

    fn shader_def() -> ShaderDefVal {
       "WORLEY".into() 
    }
}

impl ComputeNoiseShader for Worley {
    fn function_name() -> &'static str {
        "worley_2d"
    }

    fn import_path() -> &'static str {
        "bevy_compute_noise::worley"
    }

    fn struct_name() -> Option<&'static str> {
        Some("Worley")
    }
}