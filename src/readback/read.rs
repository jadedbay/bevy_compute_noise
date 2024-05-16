use bevy::{prelude::*, render::{render_asset::RenderAssets, render_graph::RenderGraph, render_resource::{Maintain, MapMode}, renderer::RenderDevice, texture::TextureFormatPixelInfo}};

use crate::{noise::ComputeNoise, noise_queue::ComputeNoiseRenderQueue, render::node::{ComputeNoiseNode, ComputeNoiseNodeState}};

use super::{util::*, ComputeNoiseReadbackSender};

pub fn map_read_texture<T: ComputeNoise>(
    render_device: Res<RenderDevice>,
    mut readback_sender: ResMut<ComputeNoiseReadbackSender>,
    images: Res<RenderAssets<Image>>,
    compute_noise_render_queue: Res<ComputeNoiseRenderQueue<T>>,
    render_graph: Res<RenderGraph>,
) {
    match render_graph.get_node::<ComputeNoiseNode<T>>(T::render_label()) {
        Ok(node) => match node.get_state() {
            ComputeNoiseNodeState::Ready => {
                for image in compute_noise_render_queue.queue.iter() {
                    if let Some(readback) = readback_sender.0.get(&image.handle) {
                        let buffer_slice = readback.buffer.as_ref().unwrap().slice(..);

                        let (s, r) = crossbeam_channel::unbounded::<()>();

                        buffer_slice.map_async(MapMode::Read, move |r| match r {
                            Ok(_) => s.send(()).expect("Failed to send map update"),
                            Err(err) => panic!("Failed to map buffer {err}"),
                        });

                        render_device.poll(Maintain::wait()).panic_on_timeout();

                        r.recv().expect("Failed to receive map_async message");

                        {
                            let buffer_view = buffer_slice.get_mapped_range();
                            let mut data = buffer_view
                                .chunks(std::mem::size_of::<u8>())
                                .map(|chunk| u8::from_ne_bytes(chunk.try_into().expect("should be u8")))
                                .collect::<Vec<u8>>();

                            let render_image = images.get(image.handle.clone()).unwrap();

                            if data.len() != ((render_image.size.x * render_image.size.y) as usize * render_image.texture_format.pixel_size()) {
                                let pixel_size = render_image.texture_format.pixel_size() as u32;
                                let initial_row_bytes = render_image.size.x as u32 * pixel_size;
                                let buffered_row_bytes = align_byte_size(render_image.size.x as u32 * pixel_size);

                                data = data
                                    .chunks_exact(buffered_row_bytes as usize)
                                    .flat_map(|row| row.iter().take(initial_row_bytes as usize))
                                    .copied()
                                    .collect();
                            }

                            readback.sender
                                .send(data)
                                .expect("Failed to send data to main world, most likely receiver was removed before sender was able to send.");
                        }

                        readback.buffer.as_ref().unwrap().unmap();

                        readback_sender.0.remove(&image.handle);
                    }
                }
            }
            _ => {}
        },
        Err(error) => {
            dbg!(error);
        }
    };
}