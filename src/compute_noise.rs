use bevy::{prelude::*, render::render_resource::ShaderType};

pub trait ComputeNoise: Sync + Send + Sized + 'static {}

#[derive(Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, ShaderType)]
#[repr(C)]
pub struct Worley2D {
    pub color: Vec4,
}
impl ComputeNoise for Worley2D {}