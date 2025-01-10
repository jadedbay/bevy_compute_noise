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
    image::ComputeNoiseSize, noise_queue::{RenderComputeNoise, ComputeNoiseBufferQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipeline
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
    let mut bind_groups: Vec<RenderComputeNoise> = Vec::new();
    for noise in queue.queue.iter() {
        let images: Option<Vec<_>> = noise.images.iter()
            .map(|handle| gpu_images.get(handle))
            .collect();
        
        if let Some(images) = images {
            let layout = match noise.size {
                ComputeNoiseSize::D2(_, _) => &pipeline.layout_2d,
                ComputeNoiseSize::D3(_, _, _) => &pipeline.layout_3d,
            };

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

            bind_groups.push(RenderComputeNoise {
                key: noise.key,
                bind_group,
                pipeline_id,
                size: noise.size,
            });
        }
    }

    render_queue
        .queue
        .extend(bind_groups.iter().cloned());
}