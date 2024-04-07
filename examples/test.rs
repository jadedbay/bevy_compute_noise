use bevy::{prelude::*, render::{mesh::shape::Cube, render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}};
use bevy_compute_noise::{compute_noise::Worley2D, ComputeNoisePlugin, ComputeNoiseQueue};
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}

fn example(
    mut worley_noise_queue: ResMut<ComputeNoiseQueue<Worley2D>>,
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

    let image_handle = images.add(image);

    worley_noise_queue.add(
        image_handle.clone(),
        Worley2D {
            color: Vec4::new(1.0, 0.0, 1.0, 1.0),
        }
    );
}

fn setup(
    mut commands: Commands,
    mut worley_noise_queue: ResMut<ComputeNoiseQueue<Worley2D>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    let image_handle = images.add(image);

    worley_noise_queue.add(
        image_handle.clone(),
        Worley2D {
            color: Vec4::new(1.0, 0.0, 0.0, 1.0),
        }
    );

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(image_handle),
            
            ..default()
        }),
        ..default()
    });
}