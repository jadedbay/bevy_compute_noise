use std::any::TypeId;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroup, BindGroupEntries, BindGroupEntry, BufferBinding, BufferInitDescriptor, BufferUsages},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::{
    image::ComputeNoiseSize, noise::{ComputeNoise, GpuComputeNoise}, noise_queue::{ComputeNoiseBindGroups, ComputeNoiseQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipelines
};

pub fn prepare_bind_groups(
    pipeline: Res<ComputeNoisePipelines>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    queue: Res<ComputeNoiseQueue>,
    render_device: Res<RenderDevice>,
    mut render_queue: ResMut<ComputeNoiseRenderQueue>,
) {
    let mut bind_groups: Vec<ComputeNoiseBindGroups> = Vec::new();
    for (image_handle, buffers, size) in queue.queue.iter() {
        if let Some(image) = gpu_images.get(image_handle) {
            let size_data = match size {
                    ComputeNoiseSize::D2(width, height) => vec![*width as f32, *height as f32],
                    ComputeNoiseSize::D3(width, height, depth) => {
                        vec![*width as f32, *height as f32, *depth as f32]
                    }
            };

            let image_size_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: None,
                contents: &bytemuck::cast_slice(size_data.as_slice()),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            });

            let image_layout = match size {
                ComputeNoiseSize::D2(_, _) => &pipeline.image_2d_layout,
                ComputeNoiseSize::D3(_, _, _) => &pipeline.image_3d_layout,
            };

            let image_bind_group = render_device.create_bind_group(
                    Some("image_bind_group".into()),
                    &image_layout,
                    &BindGroupEntries::sequential((
                        &image.texture_view,
                        BufferBinding {
                            buffer: &image_size_buffer,
                            offset: 0,
                            size: None,
                        },
                    )),
                );
            
            let noise_bind_groups: Vec<(BindGroup, TypeId)> = buffers
                .iter()
                .map(|(buffers, type_id)| {
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
                size: *size,
            });
        }
    }

    render_queue
        .queue
        .extend(bind_groups.iter().cloned());
}