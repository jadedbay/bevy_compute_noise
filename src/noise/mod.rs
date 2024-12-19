use std::any::{Any, TypeId};

use bevy::{prelude::*, reflect::{GetTypeRegistration,  Typed}, render::{render_graph::RenderLabel, render_resource::{binding_types::{uniform_buffer, uniform_buffer_sized}, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntryBuilder, BindingType, Buffer, BufferBindingType, DynamicBindGroupLayoutEntries, IntoBindGroupLayoutEntryBuilderArray, IntoIndexedBindGroupLayoutEntryBuilderArray, ShaderDefVal, ShaderRef, ShaderStages, TextureDimension}, renderer::RenderDevice}};

pub mod worley_2d;
pub mod worley_3d;
pub mod perlin_2d;
pub mod fbm;

pub use worley_2d::Worley2d;
pub use worley_3d::Worley3d;
pub use perlin_2d::Perlin2d;

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect + GetTypeRegistration + Typed {
    type Gpu: GpuComputeNoise;

    fn embed_shader(app: &mut App);
    
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer>;
    fn shader() -> ShaderRef;
    fn texture_dimension() -> TextureDimension;
    fn shader_def() -> ShaderDefVal;
    fn bind_group_layout_entries() -> Vec<BindGroupLayoutEntryBuilder> {
        vec![uniform_buffer_sized(false, None)]
    }
}
pub trait GpuComputeNoise: Sync + Send + 'static + Default + Clone {
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

pub struct ComputeNoiseSequence(pub Vec<ErasedComputeNoise>);
impl ComputeNoiseSequence {
    pub fn push_noise<T: ComputeNoise>(mut self, noise: T) -> Self {
        self.0.push(noise.into());
        self
    }

    pub fn remove_noise(&mut self, index: usize) {
        if index < self.0.len() {
            self.0.remove(index);
        } else {
            warn!("Index out of bounds: {}, trying to remove compute noise from sequence", index);
        }
    }

    pub fn edit_noise<T: ComputeNoise>(&mut self, index: usize, f: impl FnOnce(&mut T)) {
        if index >= self.0.len() {
            warn!("Index out of bounds: {}, trying to edit compute noise in sequence", index);
        }

        if let Some(noise) = self.0[index].noise_data.downcast_mut::<T>() {
            f(noise);
        } else {
            warn!("Type mismatch: trying to edit noise with incorrect type");
        }
    }
}

pub struct ComputeNoiseBuilder(Vec<ErasedComputeNoise>);
impl ComputeNoiseBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_noise<T: ComputeNoise>(mut self, noise: T) -> Self {
        self.0.push(noise.into());
        self
    }

    pub fn build(self) -> ComputeNoiseSequence {
        ComputeNoiseSequence(self.0)
    }
}

impl<T: ComputeNoise> From<T> for ComputeNoiseSequence {
    fn from(value: T) -> Self {
        Self(vec![value.into()])
    }
}

impl<T: ComputeNoise> From<T> for ErasedComputeNoise {
    fn from(value: T) -> Self {
        Self {
            noise_data: Box::new(value.clone()),
            texture_dimension: T::texture_dimension(),
            buffers_fn: Box::new(move |device| value.buffers(device)),
            type_id: TypeId::of::<T>(),
        }
    }
}