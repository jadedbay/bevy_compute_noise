use bevy::{prelude::*, render::render_resource::Buffer, utils::{HashMap, HashSet}};
use crossbeam_channel::{Receiver, Sender};

use crate::prelude::ComputeNoiseSize;

pub(crate) mod extract;
pub(crate) mod read;
pub(crate) mod util;
pub(crate) mod prepare;

#[derive(Default, Resource)]
pub struct ComputeNoiseReadback {
    pub receivers: HashMap<Handle<Image>, Receiver<Vec<u8>>>,
    pub senders: HashMap<Handle<Image>, ReadbackSender>,
    queue: HashSet<Handle<Image>>,
}

impl ComputeNoiseReadback {
    pub fn queue(&mut self, images: &mut Assets<Image>, image: Handle<Image>) {
        if !self.receivers.contains_key(&image) {
            self.add(images, image.clone());
        }

        self.queue.insert(image);
    }

    fn add(&mut self, images: &mut Assets<Image>, image: Handle<Image>) {
        let size: ComputeNoiseSize = images.get(image.clone()).unwrap().texture_descriptor.size.into();

        let (s, r) = crossbeam_channel::unbounded();
        self.receivers.insert(image.clone(), r);
        self.senders.insert(image.clone(), ReadbackSender {
            sender: s,
            size,
            buffer: None,
        });
    }

    pub fn remove(&mut self, image: &Handle<Image>) {
        self.receivers.remove(image);
        self.senders.remove(image);
    }

    pub fn receive(&self, images: &mut ResMut<Assets<Image>>, handle: Handle<Image>) -> Option<Handle<Image>> {
        if let Some(receiver) = self.receivers.get(&handle) {
            if let Ok(data) = receiver.try_recv() {
                let image = images.get_mut(handle.clone()).unwrap();
                image.data = data;

                return Some(handle);
            } else {
                dbg!("0");
            }
        } else {
            warn!("No receiver for {:?}", handle)
        }

        None
    }
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseReadbackSender(pub HashMap<Handle<Image>, ReadbackSender>);

#[derive(Clone)]
pub struct ReadbackSender {
    sender: Sender<Vec<u8>>,
    pub size: ComputeNoiseSize,
    pub buffer: Option<Buffer>,
}