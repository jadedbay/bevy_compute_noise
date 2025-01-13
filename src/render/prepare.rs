use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroupEntry, IntoBinding, PipelineCache, SpecializedComputePipelines},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::{
    noise_queue::{RenderComputeNoise, ComputeNoiseBufferQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipeline
};

pub fn prepare_render_noise(
    pipeline: Res<ComputeNoisePipeline>,
    mut pipelines: ResMut<SpecializedComputePipelines<ComputeNoisePipeline>>,
    pipeline_cache: Res<PipelineCache>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    queue: Res<ComputeNoiseBufferQueue>,
    mut render_queue: ResMut<ComputeNoiseRenderQueue>,
) {
    for sequence in queue.queue.iter() {
        let mut render_sequence = Vec::new();
        
        for noise in sequence {
            let images: Option<Vec<_>> = noise.images.iter()
                .map(|handle| gpu_images.get(handle))
                .collect();
            
            if let Some(images) = images {
                let layout = pipeline.get_layout(noise.key);
                let bind_group = render_device.create_bind_group(
                    Some("image_bind_group".into()),
                    &layout,
                    images.iter().enumerate()
                        .map(|(i, image)| BindGroupEntry {
                            binding: i as u32,
                            resource: image.texture_view.into_binding(),
                        })
                        .chain(
                            noise.buffers.iter().enumerate()
                                .map(|(i, buffer)| BindGroupEntry {
                                    binding: (images.len() + i) as u32,
                                    resource: buffer.as_entire_binding(),
                                })
                        )
                        .collect::<Vec<_>>()
                        .as_slice(),
                );

                let pipeline_id = pipelines.specialize(
                    &pipeline_cache, 
                    &pipeline, 
                    noise.key,
                );

                render_sequence.push(RenderComputeNoise {
                    key: noise.key,
                    bind_group,
                    pipeline_id,
                    size: noise.size,
                });
            }
        }

        if !render_sequence.is_empty() {
            render_queue.queue.push(render_sequence);
        }
    }
}