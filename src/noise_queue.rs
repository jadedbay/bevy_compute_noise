use std::any::TypeId;

use bevy::{
    prelude::*, render::{
        render_resource::{BindGroup, Buffer, TextureDimension, TextureView}, renderer::RenderDevice,
    }
};

use crate::{image::ComputeNoiseSize, noise::ComputeNoiseSequence, render::pipeline::ComputeNoisePipeline};

#[derive(Resource, Default)]
pub struct ComputeNoiseQueue {
    pub(crate) queue: Vec<(Handle<Image>, ComputeNoiseSequence)>,
}
impl ComputeNoiseQueue {
    pub fn add(&mut self, image: Handle<Image>, noise: ComputeNoiseSequence) {
        self.queue.push((image, noise));
    }
}

pub fn prepare_compute_noise_buffers(
    images: Res<Assets<Image>>,
    render_device: Res<RenderDevice>,
    mut noise_queue: ResMut<ComputeNoiseQueue>,
    mut noise_buffer_queue: ResMut<ComputeNoiseBufferQueue>,
) {
    for noise in &noise_queue.queue {
        let size: ComputeNoiseSize = images.get(&noise.0).unwrap().texture_descriptor.size.into();
        if noise.1.0.iter().all(|n| TextureDimension::from(size) == n.texture_dimension) {
            noise_buffer_queue.queue.push(
                ComputeNoiseBuffers {
                    image: noise.0.clone(),
                    buffers: noise.1.0.iter()
                        .map(|n| (n.type_id, n.buffers(&render_device)))
                        .collect(),
                    size,
                }
            )
        } else {
            error!("Image dimensions do not match noise dimensions - did not queue compute noise.")
        }
    }
    
    noise_queue.queue.clear();
}

#[derive(Clone)]
pub struct ComputeNoiseBindGroups {
    pub texture_view: TextureView,
    pub image_bind_group: BindGroup,
    pub noise_bind_groups: Vec<(BindGroup, ComputeNoisePipeline)>,
    pub size: ComputeNoiseSize,
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseRenderQueue {
    pub queue: Vec<ComputeNoiseBindGroups>,
}

#[derive(Clone)]
pub struct ComputeNoiseBuffers {
    pub image: Handle<Image>,
    pub buffers: Vec<(TypeId, Vec<Buffer>)>,
    pub size: ComputeNoiseSize,
}

#[derive(Resource, Clone, Default)]
pub struct ComputeNoiseBufferQueue {
    pub queue: Vec<ComputeNoiseBuffers>,
}