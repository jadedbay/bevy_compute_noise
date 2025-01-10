use std::any::{Any, TypeId};

use bevy::{prelude::*, reflect::{GetTypeRegistration,  Typed}, render::{render_resource::{Buffer, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};

pub mod worley;
pub mod perlin;
pub mod fbm;

use bytemuck::Pod;
pub use worley::{Worley, WorleyFlags};
pub use perlin::{Perlin, PerlinFlags};
pub use fbm::Fbm;

pub trait ComputeNoiseType: ComputeNoise + Pod {
    fn embed_shaders(app: &mut App);
    fn shader_2d() -> ShaderRef;
    fn shader_3d() -> ShaderRef;
    fn shader_def() -> ShaderDefVal;
}

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect + GetTypeRegistration + Typed {
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer>;
}

pub struct ErasedComputeNoise {
    noise_data: Box<dyn Any + Send + Sync>,
    buffers_fn: Box<dyn Fn(&RenderDevice) -> Vec<Buffer> + Send + Sync>,
    pub type_id: TypeId,
}

impl ErasedComputeNoise {
    pub fn as_noise<T: ComputeNoise>(&self) -> Option<&T> {
        self.noise_data.downcast_ref::<T>()
    }

    pub fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        (self.buffers_fn)(render_device)
    }
}

impl<T: ComputeNoise> From<T> for ErasedComputeNoise {
    fn from(value: T) -> Self {
        Self {
            noise_data: Box::new(value.clone()),
            buffers_fn: Box::new(move |render_device| value.buffers(render_device)),
            type_id: TypeId::of::<T>(),
        }
    }
}