use std::any::{Any, TypeId};

use bevy::{prelude::*, reflect::{GetTypeRegistration,  Typed}, render::{render_resource::{binding_types::uniform_buffer_sized, BindGroup, BindGroupLayout, BindGroupLayoutEntryBuilder, Buffer, DynamicBindGroupEntries, ShaderDefVal, ShaderRef, TextureDimension}, renderer::RenderDevice}};

pub mod worley_2d;
pub mod worley_3d;
pub mod perlin_2d;
pub mod perlin_3d;
pub mod fbm;

use bytemuck::Pod;
pub use worley_2d::{Worley2d, WorleyFlags};
// pub use worley_3d::Worley3d;
pub use perlin_2d::{Perlin2d, Perlin2dFlags};
// pub use perlin_3d::{Perlin3d, Perlin3dFlags};
pub use fbm::Fbm;

pub trait ComputeNoiseType: ComputeNoise + Pod {
    fn embed_shaders(app: &mut App);
    fn shader_2d() -> ShaderRef;
    fn shader_3d() -> ShaderRef;

    fn shader_def() -> ShaderDefVal;
    fn bind_group_layout_entries() -> Vec<BindGroupLayoutEntryBuilder>;
}

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect + GetTypeRegistration + Typed {
    fn texture_dimension() -> TextureDimension;
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer>;
}

pub struct ErasedComputeNoise {
    noise_data: Box<dyn Any + Send + Sync>,
    buffers_fn: Box<dyn Fn(&RenderDevice) -> Vec<Buffer> + Send + Sync>,
    pub texture_dimension: TextureDimension,
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
            texture_dimension: T::texture_dimension(),
            buffers_fn: Box::new(move |render_device| value.buffers(render_device)),
            type_id: TypeId::of::<T>(),
        }
    }
}