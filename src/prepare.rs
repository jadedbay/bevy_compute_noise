use bevy::{prelude::*, render::{render_asset::RenderAssets, render_resource::{BindGroupEntries, BufferBinding, BufferInitDescriptor, BufferUsages}, renderer::RenderDevice}};

use crate::{compute_noise::ComputeNoise, noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue}, pipeline::ComputeNoisePipeline};

pub fn prepare_bind_groups<T: ComputeNoise>(
    pipeline: Res<ComputeNoisePipeline<T>>,
    gpu_images: Res<RenderAssets<Image>>,
    compute_noise: Res<ComputeNoiseQueue<T>>,
    render_device: Res<RenderDevice>,
    mut compute_noise_render_queue: ResMut<ComputeNoiseRenderQueue<T>>,
) {
    let mut bind_groups = Vec::new();
    for (image_handle, noise) in compute_noise.queue.iter() {
        if let Some(image) = gpu_images.get(image_handle.clone()) {

            let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: None,
                contents: noise.as_slice(),
                usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
            });
            
            let bind_group = render_device.create_bind_group(
                Some("worley_noise_bind_group_layout".into()),
                &pipeline.layout,
                &BindGroupEntries::sequential((
                    &image.texture_view,
                    BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: None,
                    }
                )),
            );

            bind_groups.push((bind_group, image.size / 8.0));
        }
    }

    compute_noise_render_queue.queue.extend(bind_groups.iter().cloned());
}