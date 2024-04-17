use std::marker::PhantomData;

use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_resource::BindGroup}};

use crate::{compute_noise::ComputeNoise, prelude::ComputeNoiseImage};

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoiseQueue<T: ComputeNoise> {
    pub(crate) queue: Vec<(Handle<Image>, T::Gpu)>
}

impl<T: ComputeNoise> ComputeNoiseQueue<T> {
    pub fn add(&mut self, images: &mut Assets<Image>, width: u32, height: u32, noise: T) -> Handle<Image> {
        let image = ComputeNoiseImage::create_image(images, width, height);
        
        self.queue.push((image.clone(), noise.gpu_data(width, height)));

        image
    }

    pub fn add_image(&mut self, images: &mut Assets<Image>, image: Handle<Image>, noise: T) -> Handle<Image> {
        let size = images.get(image.clone()).unwrap().size();

        self.queue.push((image.clone(), noise.gpu_data(size.x, size.y)));

        image
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseRenderQueue<T: ComputeNoise> {
    pub queue: Vec<(BindGroup, BindGroup, Vec2)>,
    _phantom_data: PhantomData<T>,
}    