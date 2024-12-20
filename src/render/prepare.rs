use std::any::TypeId;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroup, BindGroupEntries, BindGroupEntry},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::{
    image::ComputeNoiseSize, noise_queue::{ComputeNoiseBindGroups, ComputeNoiseBufferQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipelines
};

pub fn prepare_bind_groups(
    pipeline: Res<ComputeNoisePipelines>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    queue: Res<ComputeNoiseBufferQueue>,
    render_device: Res<RenderDevice>,
    mut render_queue: ResMut<ComputeNoiseRenderQueue>,
) {
    let mut bind_groups: Vec<ComputeNoiseBindGroups> = Vec::new();
    for noise in queue.queue.iter() {
        if let Some(image) = gpu_images.get(&noise.image) {
            let image_layout = match noise.size {
                ComputeNoiseSize::D2(_, _) => &pipeline.image_2d_layout,
                ComputeNoiseSize::D3(_, _, _) => &pipeline.image_3d_layout,
            };

            let image_bind_group = render_device.create_bind_group(
                    Some("image_bind_group".into()),
                    &image_layout,
                    &BindGroupEntries::single(&image.texture_view),
                );
            
            let noise_bind_groups: Vec<(BindGroup, TypeId)> = noise.buffers
                .iter()
                .map(|(type_id, buffers)| {
                    let pipeline_layout = pipeline.get_pipeline(*type_id)
                        .ok_or_else(|| format!("Failed to get pipeline for type_id: {:?}", type_id))
                        .unwrap_or_else(|err| panic!("{}", err));

                    let bind_group = render_device.create_bind_group(
                        Some("noise_bind_group".into()),
                        &pipeline_layout.noise_layout,
                        buffers.iter().enumerate().map(|(i, buffer)| BindGroupEntry {
                            binding: i as u32,
                            resource: buffer.as_entire_binding(),
                        }).collect::<Vec<_>>().as_slice(),
                    );
                    (bind_group, *type_id)
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