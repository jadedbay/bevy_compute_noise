use std::marker::PhantomData;

use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_resource::BindGroup}};

use crate::{compute_noise::ComputeNoise, image::ComputeNoiseSize, prelude::ComputeNoiseImage};

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoiseQueue<T: ComputeNoise> {
    pub(crate) queue: Vec<(Handle<Image>, T::Gpu, ComputeNoiseSize)>
}

impl<T: ComputeNoise> ComputeNoiseQueue<T> {
    pub fn add(&mut self, images: &mut Assets<Image>, size: ComputeNoiseSize, noise: T) -> Handle<Image> {
        let image = ComputeNoiseImage::create_image(images, size);
        
        self.queue.push((image.clone(), noise.gpu_data(size), size));

        image
    }

    pub fn add_image(&mut self, images: &mut Assets<Image>, image: Handle<Image>, noise: T) -> Handle<Image> {
        let size = images.get(image.clone()).unwrap().texture_descriptor.size;

        self.queue.push((image.clone(), noise.gpu_data(size.into()), size.into()));

        image
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseRenderQueue<T: ComputeNoise> {
    pub queue: Vec<ComputeNoiseBindGroups>,
    _phantom_data: PhantomData<T>,
}

#[derive(Clone)]
pub struct ComputeNoiseBindGroups {
    pub image_bind_group: BindGroup,
    pub noise_bind_group: BindGroup,
    pub size: ComputeNoiseSize,
}