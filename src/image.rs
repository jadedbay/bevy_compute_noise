use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}};

pub struct ComputeNoiseImage;

impl ComputeNoiseImage {
    pub fn create_image(size: ComputeNoiseSize) -> Image {
        let mut image = 
            Image::new_fill(
                size.into(),
                size.into(),
                &[0, 0, 0, 0],
                TextureFormat::Rgba8Unorm,
                RenderAssetUsages::all(),
            );

        image.texture_descriptor.usage = TextureUsages::COPY_DST
            | TextureUsages::COPY_SRC
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;

        image
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComputeNoiseSize {
    D2(u32, u32),
    D3(u32, u32, u32),
}

impl ComputeNoiseSize {
    pub fn width(&self) -> u32 {
        match self {
            Self::D2(width, _) => *width,
            Self::D3(width, _, _) => *width,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::D2(_, height) => *height,
            Self::D3(_, height, _) => *height,
        }
    }

    pub fn depth(&self) -> u32 {
        match self {
            Self::D2(_, _) => 1,
            Self::D3(_, _, depth) => *depth,
        }
    }

    pub fn data_len(&self) -> usize {
        match self {
            Self::D2(width, height) => (width * height) as usize,
            Self::D3(width, height, depth) => (width * height * depth) as usize, 
        }
    }

    pub(crate) fn workgroup_count(&self) -> (u32, u32, u32) {
        match self {
            Self::D2(width, height) => (width / 8, height / 8, 1),
            Self::D3(width, height, depth) => (width / 8, height / 8, depth / 8),
        }
    }
}

impl From<ComputeNoiseSize> for Extent3d {
    fn from(value: ComputeNoiseSize) -> Self {
        match value {
            ComputeNoiseSize::D2(width, height) => {
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                }
            },
            ComputeNoiseSize::D3(width, height, depth) => {
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: depth,
                }
            }
        }
    }
}

impl From<Extent3d> for ComputeNoiseSize {
    fn from(value: Extent3d) -> Self {
        if value.depth_or_array_layers == 1 {
            ComputeNoiseSize::D2(value.width, value.height)
        } else {
            ComputeNoiseSize::D3(value.width, value.height, value.depth_or_array_layers)
        }
    }
}

impl From<ComputeNoiseSize> for TextureDimension {
    fn from(value: ComputeNoiseSize) -> Self {
        match value {
            ComputeNoiseSize::D2(_, _) => TextureDimension::D2,
            ComputeNoiseSize::D3(_, _, _) => TextureDimension::D3,
        }
    }
}