use std::any::TypeId;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroup, BindGroupEntries, BindGroupEntry, BufferDescriptor, BufferUsages, IntoBinding, PipelineCache, SpecializedComputePipelines, TextureDimension},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::{
    image::ComputeNoiseSize, noise::{ComputeNoiseType, Fbm}, noise_queue::{ComputeNoiseBindGroups, ComputeNoiseBufferQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipelines
};

use super::pipeline::ComputeNoisePipelineKey;

// use super::pipeline::{ComputeNoiseFbmPipeline, ComputeNoisePipeline, FbmPipelineKey};

pub fn prepare_bind_groups(
    pipelines: Res<ComputeNoisePipelines>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    queue: Res<ComputeNoiseBufferQueue>,
    mut render_queue: ResMut<ComputeNoiseRenderQueue>,
) {
    let mut bind_groups: Vec<ComputeNoiseBindGroups> = Vec::new();
    for noise in queue.queue.iter() {
        let images: Option<Vec<_>> = noise.images.iter()
            .map(|handle| gpu_images.get(handle))
            .collect();
        
        if let Some(images) = images {
            let layout = match noise.size {
                ComputeNoiseSize::D2(_, _) => &pipelines.layout_2d,
                ComputeNoiseSize::D3(_, _, _) => &pipelines.layout_3d,
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

            bind_groups.push(ComputeNoiseBindGroups {
                key: noise.key,
                bind_group,
                size: noise.size,
            });
        }
    }

    render_queue
        .queue
        .extend(bind_groups.iter().cloned());
}

pub fn prepare_compute_noise_pipelines(
    mut compute_noise_pipelines: ResMut<SpecializedComputePipelines<ComputeNoisePipelines>>,
    pipeline: Res<ComputeNoisePipelines>,
    queue: Res<ComputeNoiseBufferQueue>,
    mut render_queue: ResMut<ComputeNoiseRenderQueue>,
    pipeline_cache: Res<PipelineCache>,
) {
    for item in &queue.queue {
        render_queue.pipeline_ids.push(
            compute_noise_pipelines.specialize(
                &pipeline_cache, 
                &pipeline, 
                item.key,
            )
        );
    }
}
