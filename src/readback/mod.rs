use bevy::{prelude::*, render::{render_resource::{Buffer, BufferDescriptor, BufferUsages, TextureFormat}, renderer::RenderDevice, texture::TextureFormatPixelInfo}, utils::HashMap};
use crossbeam_channel::{Receiver, Sender};

use crate::prelude::ComputeNoiseSize;

use self::util::get_aligned_size;

pub(crate) mod extract;
pub(crate) mod read;
pub(crate) mod util;

#[derive(Default, Resource)]
pub struct ComputeNoiseReadback(pub(crate) HashMap<Handle<Image>, ComputeNoiseSize>);

#[derive(Default, Resource)]
pub struct ComputeNoiseReadbackReceiver {
    pub images: HashMap<Handle<Image>, Receiver<Vec<u8>>>,
}

impl ComputeNoiseReadbackReceiver {
    pub fn receive(&mut self, images: &mut ResMut<Assets<Image>>, handle: Handle<Image>) -> Option<Handle<Image>> {
        if let Some(receiver) = self.images.get(&handle) {
            if let Ok(data) = receiver.try_recv() {
                let image = images.get_mut(handle.clone()).unwrap();
                image.data = data;

                self.images.remove(&handle);
                return Some(handle);
            }
        } else {
            warn!("No receiver for {:?}", handle)
        }

        None
    }
}

#[derive(Default, Resource, Clone)]
pub struct ComputeNoiseReadbackSender {
    pub images: HashMap<Handle<Image>, (Sender<Vec<u8>>, Buffer)>
}

pub fn add_readback(
    mut readback: ResMut<ComputeNoiseReadback>,
    mut readback_receiver: ResMut<ComputeNoiseReadbackReceiver>,
    mut readback_sender: ResMut<ComputeNoiseReadbackSender>,
    render_device: Res<RenderDevice>,
) {
    for image in readback.0.iter() {
        let (s, r) = crossbeam_channel::unbounded();
        readback_receiver.images.insert(image.0.clone(), r);
        readback_sender.images.insert(
            image.0.clone(),
            (
                s, 
                render_device.create_buffer(&BufferDescriptor {
                    label: Some("readback_buffer"),
                    size: get_aligned_size(image.1.width(), image.1.height(), TextureFormat::R8Unorm.pixel_size() as u32) as u64,
                    usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
            )
        );
    }

    readback.0.clear();
}