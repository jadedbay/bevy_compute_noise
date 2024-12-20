use std::any::TypeId;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroup, BindGroupEntries, BindGroupEntry, PipelineCache, SpecializedComputePipelines},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::{
    image::ComputeNoiseSize, noise::{ComputeNoiseType, Fbm}, noise_queue::{ComputeNoiseBindGroups, ComputeNoiseBufferQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipelines
};

use super::pipeline::{ComputeNoiseFbmPipeline, ComputeNoisePipeline, FbmPipelineKey};

pub fn prepare_bind_groups(
    pipelines: Res<ComputeNoisePipelines>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    queue: Res<ComputeNoiseBufferQueue>,
    render_device: Res<RenderDevice>,
    mut render_queue: ResMut<ComputeNoiseRenderQueue>,
) {
    let mut bind_groups: Vec<ComputeNoiseBindGroups> = Vec::new();
    for noise in queue.queue.iter() {
        if let Some(image) = gpu_images.get(&noise.image) {
            let image_layout = match noise.size {
                ComputeNoiseSize::D2(_, _) => &pipelines.image_2d_layout,
                ComputeNoiseSize::D3(_, _, _) => &pipelines.image_3d_layout,
            };

            let image_bind_group = render_device.create_bind_group(
                    Some("image_bind_group".into()),
                    &image_layout,
                    &BindGroupEntries::single(&image.texture_view),
                );
            
            let noise_bind_groups: Vec<(BindGroup, ComputeNoisePipeline)> = noise.buffers
                .iter()
                .map(|(type_id, buffers)| {
                    let pipeline = pipelines.get_pipeline(*type_id)
                        .ok_or_else(|| format!("Failed to get pipeline for type_id: {:?}", type_id)) // Sometimes get error here TODO: fix it
                        .unwrap_or_else(|err| panic!("{}", err));

                    let bind_group = render_device.create_bind_group(
                        Some("noise_bind_group".into()),
                        &pipeline.noise_layout,
                        buffers.iter().enumerate().map(|(i, buffer)| BindGroupEntry {
                            binding: i as u32,
                            resource: buffer.as_entire_binding(),
                        }).collect::<Vec<_>>().as_slice(),
                    );
                    (bind_group, pipeline.clone())
                })
                .collect();

            bind_groups.push(ComputeNoiseBindGroups {
                image_bind_group,
                noise_bind_groups,
                size: noise.size,
            });
        }
    }

    render_queue
        .queue
        .extend(bind_groups.iter().cloned());
}

pub fn prepare_fbm_pipeline<T: ComputeNoiseType>(
    fbm_pipeline: Res<ComputeNoiseFbmPipeline>,
    mut fbm_pipelines: ResMut<SpecializedComputePipelines<ComputeNoiseFbmPipeline>>,
    mut compute_noise_pipelines: ResMut<ComputeNoisePipelines>,
    queue: Res<ComputeNoiseBufferQueue>,
    pipeline_cache: Res<PipelineCache>,
) {
    let fbm_type_id = TypeId::of::<Fbm<T>>();
    
    if compute_noise_pipelines.get_pipeline(fbm_type_id).is_some() {
        return;
    }

    if !queue.queue.iter()
        .any(|sequence| sequence.buffers
            .iter()
            .any(|noise| noise.0 == fbm_type_id)
        ) { return; }

    let noise_type_id = TypeId::of::<T>();
    let noise_layout = fbm_pipeline.type_data.get(&noise_type_id)
        .expect("Missing FBM pipeline type data")
        .2
        .clone();

    let pipeline = ComputeNoisePipeline {
        noise_layout,
        pipeline_id: fbm_pipelines.specialize(
            &pipeline_cache,
            &fbm_pipeline,
            FbmPipelineKey { noise_type_id }
        )
    };

    compute_noise_pipelines.add_pipeline::<Fbm<T>>(pipeline);
}
