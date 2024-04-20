use bevy::{prelude::*, render::{render_asset::RenderAssets, render_graph::RenderGraph, render_resource::{BindGroupEntries, BufferBinding, BufferInitDescriptor, BufferUsages}, renderer::RenderDevice}};

use crate::{node::{ComputeNoiseNode, ComputeNoiseNodeState}, compute_noise::{ComputeNoise, GpuComputeNoise}, image::ComputeNoiseSize, noise_queue::{ComputeNoiseBindGroups, ComputeNoiseQueue, ComputeNoiseRenderQueue}, pipeline::ComputeNoisePipeline};

pub fn prepare_bind_groups<T: ComputeNoise>(
    pipeline: Res<ComputeNoisePipeline<T>>,
    gpu_images: Res<RenderAssets<Image>>,
    compute_noise: Res<ComputeNoiseQueue<T>>,
    render_device: Res<RenderDevice>,
    render_graph: Res<RenderGraph>,
    mut compute_noise_render_queue: ResMut<ComputeNoiseRenderQueue<T>>,
) {
    let mut bind_groups: Vec<ComputeNoiseBindGroups> = Vec::new();
    for (image_handle, noise, size) in compute_noise.queue.iter() {
        if let Some(image) = gpu_images.get(image_handle.clone()) {
            let size_data = match size {
                ComputeNoiseSize::D2(width, height) => vec![*width as f32, *height as f32],
                ComputeNoiseSize::D3(width, height, depth) => vec![*width as f32, *height as f32, *depth as f32],
            };

            let image_size_buffer = render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: None,
                    contents: &bytemuck::cast_slice(size_data.as_slice()),
                    usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
                }
            );

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
                image_bind_group,
                noise_bind_group,
                size: *size,
            });
        }
    }

    let mut index = 0;
    match render_graph.get_node::<ComputeNoiseNode<T>>(T::render_label()) {
        Ok(node) => {
            match node.get_state() {
                ComputeNoiseNodeState::Ready(0) => index = 1,
                _ => {}
            }
        },
        Err(error) => { dbg!(error); },
    };
    compute_noise_render_queue.queue[index].extend(bind_groups.iter().cloned());
}

