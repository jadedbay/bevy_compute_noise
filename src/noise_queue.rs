use std::marker::PhantomData;

use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_asset::RenderAssetUsages, render_resource::{BindGroup, Extent3d, TextureDimension, TextureFormat, TextureUsages}}};

use crate::{compute_noise::{ComputeNoise, GpuComputeNoise}, prelude::Worley2D};

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

pub enum GpuNoiseChannels {
    R(Box<dyn GpuComputeNoise>),
    RG(Box<dyn GpuComputeNoise>, Box<dyn GpuComputeNoise>),
    RGBA(Box<dyn GpuComputeNoise>, Box<dyn GpuComputeNoise>, Box<dyn GpuComputeNoise>, Option<Box<dyn GpuComputeNoise>>)
}

pub enum NoiseChannels {
    R(Box<dyn ComputeNoise>),
    RG(Box<dyn ComputeNoise>, Box<dyn ComputeNoise>),
    RGBA(Box<dyn ComputeNoise>, Box<dyn ComputeNoise>, Box<dyn ComputeNoise>, Option<Box<dyn ComputeNoise>>)
}

pub struct CNQueue {
    pub queue: Vec<(Handle<Image>, GpuNoiseChannels)>
}

impl CNQueue {
    pub fn add(
        &mut self,
        images: &mut Assets<Image>, 
        width: u32, 
        height: u32, 
        noise: NoiseChannels
    ) -> Handle<Image> {
        let image_handle = Self::create_image(images, width, height, noise);

        self.queue.push(match noise {
            NoiseChannels::R(noise) => 
                GpuNoiseChannels::R(noise.gpu_data(width, height)),
            NoiseChannels::RG(noise_r, noise_g) => 
                GpuNoiseChannels::RG(noise_r.gpu_data(width, height), noise_g.gpu_data(width, height)),
            NoiseChannels::RGBA(noise_r, noise_g, noise_b, noise_a) => 
                GpuNoiseChannels::RGBA(noise_r.gpu_data(width, height), noise_g.gpu_data(width, height), noise_b.gpu_data(width, height), match noise_a {
                    Some(noise_a) => Some(noise_a.gpu_data(width, height)),
                    None => None,
                })
        });

        image_handle
    }

    pub fn create_image(images: &mut Assets<Image>, width: u32, height: u32, channels: NoiseChannels) -> Handle<Image> {
        let texture_format = match channels {
            NoiseChannels::R(_) => TextureFormat::R8Unorm,
            NoiseChannels::RG(_, _) => TextureFormat::Rg8Unorm,
            NoiseChannels::RGBA(_, _, _, _) => TextureFormat::Rgba8Unorm,
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

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
