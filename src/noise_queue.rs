use std::{any::{Any, TypeId}, marker::PhantomData};

use bevy::{
    prelude::*, render::{
        extract_resource::ExtractResource, render_resource::{BindGroup, Buffer, TextureDimension}, renderer::RenderDevice,
    }
};

use crate::{image::{ComputeNoiseFormat, ComputeNoiseSize}, noise::{ComputeNoise, ComputeNoiseSequence, ErasedComputeNoise, GpuComputeNoise}, prelude::ComputeNoiseImage, render};

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoiseQueue {
    pub(crate) queue: Vec<(Handle<Image>, Vec<(Vec<Buffer>, TypeId)>, ComputeNoiseSize)>
}

impl ComputeNoiseQueue {
    pub fn add(
        &mut self,
        images: &mut Assets<Image>,
        size: ComputeNoiseSize,
        format: ComputeNoiseFormat,
        noise: ComputeNoiseSequence,
        render_device: &RenderDevice,
    ) -> Handle<Image> {
        let image = images.add(ComputeNoiseImage::create_image(size, format));
        if noise.0.iter().all(|n| TextureDimension::from(size) == n.texture_dimension) {
            self.queue.push((
                image.clone(), 
                noise.0.iter()
                    .map(|n| (n.buffers(render_device, &size), n.type_id))
                    .collect(),
                size
            ));
        } else {
            error!("Image dimensions do not match noise dimensions - created image but did not queue compute noise.")
        }

        image
    }

    pub fn add_image(
        &mut self,
        images: &mut Assets<Image>,
        render_device: &RenderDevice,
        image: Handle<Image>,
        noise: ComputeNoiseSequence,
    ) -> Handle<Image> {
        let size: ComputeNoiseSize = images.get(&image).unwrap().texture_descriptor.size.into();
        if noise.0.iter().all(|n| TextureDimension::from(size) == n.texture_dimension) {
            self.queue.push((
                image.clone(), 
                noise.0.iter()
                    .map(|n| (n.buffers(render_device, &size), n.type_id))
                    .collect(),
                size
            ));
        } else {
            error!("Image dimensions do not match noise dimensions - did not queue compute noise.")
        }

        image
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

#[derive(Clone)]
pub struct ComputeNoiseBindGroups {
    pub image_bind_group: BindGroup,
    pub noise_bind_groups: Vec<(BindGroup, TypeId)>,
    pub size: ComputeNoiseSize,
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseRenderQueue {
    pub queue: Vec<ComputeNoiseBindGroups>,
}