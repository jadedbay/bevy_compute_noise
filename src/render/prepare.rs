use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroupEntries, BufferBinding, BufferInitDescriptor, BufferUsages},
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::{
    image::ComputeNoiseSize, noise::{ComputeNoise, GpuComputeNoise}, noise_queue::{ComputeNoiseBindGroups, ComputeNoiseQueue, ComputeNoiseRenderQueue}, render::pipeline::ComputeNoisePipeline
};

pub fn prepare_bind_groups<T: ComputeNoise>(
    pipeline: Res<ComputeNoisePipeline<T>>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    compute_noise: Res<ComputeNoiseQueue<T>>,
    render_device: Res<RenderDevice>,
    mut compute_noise_render_queue: ResMut<ComputeNoiseRenderQueue<T>>,
) {
    let mut bind_groups: Vec<ComputeNoiseBindGroups> = Vec::new();
    for (image_handle, noise, size) in compute_noise.queue.iter() {
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

            let noise_bind_group = noise.bind_group(&render_device, &pipeline.noise_layout);

            bind_groups.push(ComputeNoiseBindGroups {
                handle: image_handle.clone(),
                image_bind_group,
                noise_bind_group,
                size: *size,
            });
        }
    }

    compute_noise_render_queue
        .queue
        .extend(bind_groups.iter().cloned());
}