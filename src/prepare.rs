use bevy::{prelude::*, render::{render_asset::RenderAssets, render_resource::{BindGroup, BindGroupEntries, BufferBinding, BufferInitDescriptor, BufferUsages}, renderer::RenderDevice}};

use crate::{compute_noise::{ComputeNoise, GpuComputeNoise}, noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue}, pipeline::ComputeNoisePipeline};

pub fn prepare_bind_groups<T: ComputeNoise>(
    pipeline: Res<ComputeNoisePipeline<T>>,
    gpu_images: Res<RenderAssets<Image>>,
    compute_noise: Res<ComputeNoiseQueue<T>>,
    render_device: Res<RenderDevice>,
    mut compute_noise_render_queue: ResMut<ComputeNoiseRenderQueue<T>>,
) {
    let mut bind_groups: Vec<(BindGroup, BindGroup, Vec2)> = Vec::new();
    for (image_handle, noise) in compute_noise.queue.iter() {
        if let Some(image) = gpu_images.get(image_handle.clone()) {

            let image_size_buffer = render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                        label: None,
                        contents: &bytemuck::cast_slice(&[image.size]),
                        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                });

            let image_bind_group = render_device.create_bind_group(
                Some("image_bind_group".into()),
                &pipeline.image_layout,
                &BindGroupEntries::sequential((
                    &image.texture_view,
                    BufferBinding {
                        buffer: &image_size_buffer,
                        offset: 0,
                        size: None,
                    },
                )),
            );

            let noise_bind_group = noise.to_bind_group(&render_device, &pipeline.noise_layout);

            bind_groups.push((image_bind_group, noise_bind_group, image.size / 8.0));
        }
    }

    compute_noise_render_queue.queue.extend(bind_groups.iter().cloned());
}

