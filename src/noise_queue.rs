use std::any::TypeId;

use bevy::{
    prelude::*, render::{
        render_resource::{BindGroup, Buffer, CachedComputePipelineId, TextureDimension, TextureView}, renderer::RenderDevice,
    }
};

use crate::{image::ComputeNoiseSize, noise::{ErasedComputeNoise, Fbm}, render::pipeline::ComputeNoisePipelineKey};

#[derive(Resource, Default)]
pub struct ComputeNoiseQueue {
    pub queue: Vec<(Vec<Handle<Image>>, ErasedComputeNoise)>,
}
impl ComputeNoiseQueue {
    pub fn write(&mut self, image: Handle<Image>, noise: ErasedComputeNoise) {
        self.queue.push((
            vec![image],
            noise,
        ));
    }

    pub fn modify(&mut self, input: Handle<Image>, output: Handle<Image>, modification: ErasedComputeNoise) {
        self.queue.push((
            vec![input, output],
            modification,
        ))
    }

    pub fn combine(&mut self, input1: Handle<Image>, input2: Handle<Image>, output: Handle<Image>, op: ErasedComputeNoise) {
        self.queue.push((
            vec![input1, input2, output],
            op,
        ))
    }
}

pub fn prepare_compute_noise_buffers(
    images: Res<Assets<Image>>,
    render_device: Res<RenderDevice>,
    mut noise_queue: ResMut<ComputeNoiseQueue>,
    mut noise_buffer_queue: ResMut<ComputeNoiseBufferQueue>,
) {
    for item in &noise_queue.queue {
        let sizes: Vec<ComputeNoiseSize> = item.0.iter()
            .map(|image_handle| {
                images.get(image_handle).unwrap().texture_descriptor.size.into()
            })
            .collect();

        if !sizes.windows(2).all(|window| TextureDimension::from(window[0]) == TextureDimension::from(window[1])) {
            error!("Not all images have the same dimension - did not queue compute noise.");
            continue;
        }

        noise_buffer_queue.queue.push(ComputeNoiseBuffers {
            key: ComputeNoisePipelineKey {
                type_id: item.1.type_id,
                dimension: sizes[0].into(),
            },
            images: item.0.clone(),
            buffers: item.1.buffers(&render_device),
            size: *sizes.last().unwrap(),
        });
    }
    
    noise_queue.queue.clear();
}

#[derive(Clone)]
pub struct ComputeNoiseBindGroups {
    pub key: ComputeNoisePipelineKey,
    pub bind_group: BindGroup,
    pub size: ComputeNoiseSize,
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseRenderQueue {
    pub queue: Vec<ComputeNoiseBindGroups>,
    pub pipeline_ids: Vec<CachedComputePipelineId>,
}

#[derive(Clone)]
pub struct ComputeNoiseBuffers {
    pub key: ComputeNoisePipelineKey,
    pub images: Vec<Handle<Image>>,
    pub buffers: Vec<Buffer>,
    pub size: ComputeNoiseSize,
}

#[derive(Resource, Clone, Default)]
pub struct ComputeNoiseBufferQueue {
    pub queue: Vec<ComputeNoiseBuffers>,
}