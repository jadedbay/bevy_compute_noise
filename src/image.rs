use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}}};

pub struct ComputeNoiseImage;

impl ComputeNoiseImage {
    pub fn create_image(images: &mut Assets<Image>, width: u32, height: u32, depth: u32) -> Handle<Image> {
        let mut image = 
            Image::new_fill(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: depth,
                },
            TextureDimension::D2,
            &[0],
            TextureFormat::R8Unorm,
            RenderAssetUsages::all(),
        );

        image.texture_descriptor.usage = TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;

        images.add(image)
    }
}