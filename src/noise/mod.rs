use std::any::{Any, TypeId};

use bevy::{reflect::{FromReflect, GetTypeRegistration, TypePath, Typed}, render::{render_resource::Buffer, renderer::RenderDevice}};

use crate::render::pipeline::NoiseOp;

pub mod generators;
pub mod modifiers;

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect + GetTypeRegistration + Typed {
    const NOISE_OP: NoiseOp;

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