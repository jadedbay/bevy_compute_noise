use bevy::{prelude::*, render::{render_asset::{RenderAssetUsages, RenderAssets}, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}};
use bevy_compute_noise::{ComputeNoise, ComputeNoisePlugin, WorleySettings};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin
        ))
        .add_systems(Startup, test)
        .run()
}

fn test(
    gpu_images: Res<RenderAssets<Image>>,
) {
    let test = gpu_images.get(Handle::default());
}

fn example(
    mut noise_gen: ResMut<ComputeNoise>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut image = 
        Image::new_fill(
            Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );

    image.texture_descriptor.usage = TextureUsages::COPY_DST
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::TEXTURE_BINDING;

    noise_gen.generate_worley(
        images.add(image),
        WorleySettings {
            point_count: 10,
        }
    );
}