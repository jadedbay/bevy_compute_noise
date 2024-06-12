use std::marker::PhantomData;

use bevy::{
    prelude::*, render::{
        extract_resource::ExtractResource, render_resource::{BindGroup, TextureDimension},
    }
};

use crate::{image::{ComputeNoiseFormat, ComputeNoiseSize}, noise::ComputeNoise, prelude::ComputeNoiseImage};

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoiseQueue<T: ComputeNoise> {
    pub(crate) queue: Vec<(Handle<Image>, T::Gpu, ComputeNoiseSize)>,
}

impl<T: ComputeNoise> ComputeNoiseQueue<T> {
    pub fn add(
        &mut self,
        images: &mut Assets<Image>,
        size: ComputeNoiseSize,
        format: ComputeNoiseFormat,
        noise: T,
    ) -> Handle<Image> {
        let image = images.add(ComputeNoiseImage::create_image(size, format));
        if TextureDimension::from(size) == T::texture_dimension() {
            self.queue.push((image.clone(), noise.gpu_data(size), size));
        } else {
            error!("Image dimensions do not match noise dimensions - created image but did not queue compute noise.")
        }

        image
    }

    pub fn add_image(
        &mut self,
        images: &mut Assets<Image>,
        image: Handle<Image>,
        noise: T,
    ) -> Handle<Image> {
        let size: ComputeNoiseSize = images.get(image.clone()).unwrap().texture_descriptor.size.into();
        if TextureDimension::from(size) == T::texture_dimension() {
            self.queue.push((image.clone(), noise.gpu_data(size), size));
        } else {
            error!("Image dimensions do not match noise dimensions - did not queue compute noise.")
        }

        image
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[derive(Default, Resource, Debug)]
pub(crate) struct ComputeNoiseRenderQueue<T: ComputeNoise> {
    pub queue: Vec<ComputeNoiseBindGroups>,
    _phantom_data: PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct ComputeNoiseBindGroups {
    pub handle: Handle<Image>,
    pub image_bind_group: BindGroup,
    pub noise_bind_group: BindGroup,
    pub size: ComputeNoiseSize,
}