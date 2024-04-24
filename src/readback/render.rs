use bevy::{prelude::*, render::{render_asset::RenderAssets, render_graph::RenderGraph, render_resource::{CommandEncoderDescriptor, Extent3d, ImageCopyBuffer, Maintain, MapMode}, renderer::{RenderDevice, RenderQueue}, texture::TextureFormatPixelInfo}};

use crate::{noise::ComputeNoise, noise_queue::ComputeNoiseRenderQueue, render::node::{ComputeNoiseNode, ComputeNoiseNodeState}};

use super::{util::*, ComputeNoiseReadbackSender};

pub fn readback_texture<T: ComputeNoise>(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut readback_sender: ResMut<ComputeNoiseReadbackSender>,
    images: Res<RenderAssets<Image>>,
    compute_noise_render_queue: Res<ComputeNoiseRenderQueue<T>>,
    render_graph: Res<RenderGraph>,
) {
    match render_graph.get_node::<ComputeNoiseNode<T>>(T::render_label()) {
        Ok(node) => match node.get_state() {
            ComputeNoiseNodeState::Ready => {
                for image in compute_noise_render_queue.queue.iter() {
                    if let Some(readback) = readback_sender.images.get(&image.handle) {

                        let render_image = images.get(image.handle.clone()).unwrap();

                        let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor::default());

                        encoder.copy_texture_to_buffer(
                            render_image.texture.as_image_copy(), 
                            ImageCopyBuffer {
                                buffer: &readback.1,
                                layout: layout_data(render_image.size.x as u32, render_image.size.y as u32, render_image.texture_format)
                            },
                            Extent3d {
                                width: render_image.size.x as u32,
                                height: render_image.size.y as u32,
                                ..default()
                            }
                        );
                        render_queue.submit([encoder.finish()]);

                        let buffer_slice = readback.1.slice(..);

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

                            dbg!(&data);

                            readback.0
                                .send(data)
                                .expect("Failed to send data to main world, most likely a new noise was added to the queue before old one could be sent back.");
                        }

                        readback.1.unmap();

                        readback_sender.images.remove(&image.handle);
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