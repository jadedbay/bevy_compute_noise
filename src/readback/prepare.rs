use bevy::{prelude::*, render::{render_resource::{BufferDescriptor, BufferUsages, TextureFormat}, renderer::RenderDevice, texture::TextureFormatPixelInfo}};

use super::{util::get_aligned_size, ComputeNoiseReadbackSender};

pub fn prepare_readback_buffers(
    render_device: Res<RenderDevice>,
    mut readback_sender: ResMut<ComputeNoiseReadbackSender>,
) {
    for image in readback_sender.0.values_mut() {
        image.buffer = Some(render_device.create_buffer(&BufferDescriptor {
            label: Some("readback_buffer"),
            size: get_aligned_size(image.size.width(), image.size.height(), TextureFormat::R8Unorm.pixel_size() as u32) as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
    }
}