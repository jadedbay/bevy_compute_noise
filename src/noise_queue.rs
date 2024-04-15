use std::marker::PhantomData;

use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_asset::RenderAssetUsages, render_resource::{BindGroup, Extent3d, TextureDimension, TextureFormat, TextureUsages}}};

use crate::{compute_noise::ComputeNoise, prelude::Worley2D};

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoiseQueue<T: ComputeNoise> {
    pub(crate) queue: Vec<(Handle<Image>, T)>
}

impl<T: ComputeNoise> ComputeNoiseQueue<T> {
    pub fn add(&mut self, images: &mut Assets<Image>, width: u32, height: u32, settings: T::Settings) -> Handle<Image> {
        let image = Self::create_image(images, width, height);
        
        self.queue.push((image.clone(), T::new(width, height, settings)));

        image
    }

    pub fn add_rgba<
        R: ComputeNoise, 
        G: ComputeNoise, 
        B: ComputeNoise, 
        A: ComputeNoise
        >(&mut self, images: &mut Assets<Image>, width: u32, height: u32, settings: (Option<R::Settings>, Option<G::Settings>, Option<B::Settings>, Option<A::Settings>)) -> Handle<Image> {
            let image = Self::create_image(images, width, height);
        
            //self.queue.push((image.clone(), T::new(width, height, settings)));

            image
        }

    pub fn add_image(&mut self, images: &mut Assets<Image>, image: Handle<Image>, settings: T::Settings) -> Handle<Image> {
        let size = images.get(image.clone()).unwrap().size();

        self.queue.push((image.clone(), T::new(size.x, size.y, settings)));

        image
    }

    pub fn create_image(images: &mut Assets<Image>, width: u32, height: u32) -> Handle<Image> {
        let mut image = 
            Image::new_fill(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            TextureDimension::D2,
            &[0],
            TextureFormat::R8Unorm,
            RenderAssetUsages::RENDER_WORLD,
        );

        image.texture_descriptor.usage = TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;

        images.add(image)
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

pub enum Noise {
    Worley2D(Worley2D),
}

pub struct CNQueue {
    pub queue: Vec<(Handle<Image>, (Option<Box<dyn ComputeNoise>>, Option<Box<dyn ComputeNoise>>, Option<Box<dyn ComputeNoise>>, Option<Box<dyn ComputeNoise>>))>
}

impl CNQueue {
    pub fn add(
        &mut self,
        images: &mut Assets<Image>, 
        width: u32, 
        height: u32, 
        settings: (
            Option<Box<dyn ComputeNoise>>, 
            Option<Box<dyn ComputeNoise>>, 
            Option<Box<dyn ComputeNoise>>, 
            Option<Box<dyn ComputeNoise>>
        )
    ) -> Handle<Image> {
        let num_channels = settings.0.is_some() as u8
            + settings.1.is_some() as u8
            + settings.2.is_some() as u8
            + settings.3.is_some() as u8;
        
        let valid_channels = match num_channels {
            1 | 2 => num_channels,
            _ => 4,
        };

        let image_handle = Self::create_image(images, width, height, valid_channels);

        image_handle
    }

    pub fn create_image(images: &mut Assets<Image>, width: u32, height: u32, channels: u8) -> Handle<Image> {
        let texture_format = match channels {
            1 => TextureFormat::R8Unorm,
            2 => TextureFormat::Rg8Unorm,
            _ => TextureFormat::Rgba8Unorm,
        };

        let mut image = 
            Image::new_fill(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            TextureDimension::D2,
            &[0],
            texture_format,
            RenderAssetUsages::RENDER_WORLD,
        );

        image.texture_descriptor.usage = TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;

        images.add(image)
    }
}