use bevy::{core_pipeline::tonemapping::Tonemapping, image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor}, prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}};
use bevy_compute_noise::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<ImageMaterial>::default(),
            ComputeNoisePlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        // .add_systems(Update, update_noise)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ImageMaterial>>,
    mut noise_queue: ResMut<ComputeNoiseQueue>
) {
    let mut image = ComputeNoiseImage::create_image(ComputeNoiseSize::D2(1024, 1024), ComputeNoiseFormat::Rgba);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        ..default()
    });
    let handle = images.add(image);


    let mut quad = Rectangle::default().mesh().build();
    if let Some(uvs) = quad.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        if let VertexAttributeValues::Float32x2(uvs) = uvs {
            for uv in uvs.iter_mut() {
                *uv = [uv[0] * 2.0, uv[1] * 2.0];
            }
        }
    }

    // noise_queue.add(
    //     handle.clone(),
    //     Perlin2d {
    //         seed: 1,
    //         frequency: 4.,
    //         invert: false,
    //     }.into(),
    // );

    noise_queue.add(
        handle.clone(),
        Fbm::<Worley2d> {
            noise: Worley2d {
                seed: 5,
                frequency: 5.5,
                ..default()
            },
            octaves: 2,
            lacunarity: 2.0,
            persistence: 0.5,
        }.into(),
    );

    commands.spawn((
        Mesh2d(meshes.add(quad)),
        Transform::default().with_scale(Vec3::splat(512.)),
        // MeshMaterial2d(materials.add(ImageMaterial {
        //     image: handle.clone(),
        // })),
        MeshMaterial2d(materials.add(ImageMaterial {
            image: handle.clone(),
        }))
    ));

    commands.spawn((Camera2d, Tonemapping::None));
}

// fn update_noise(
//     mut noise_queue: ResMut<ComputeNoiseQueue>,
//     query: Query<&MeshMaterial2d<ColorMaterial>>,
//     keys: Res<ButtonInput<KeyCode>>,
//     mut local: Local<u32>,
//     materials: Res<Assets<ColorMaterial>>,
// ) {
//     if keys.just_pressed(KeyCode::Space) {
//         for material in query.iter() {
//             noise_queue.add(
//                 materials.get(&material.0).unwrap().texture.as_ref().unwrap().clone(),
//                     Perlin2d {
//                         seed: *local,
//                         frequency: 4.,
//                         invert: false,
//                     }.into(),
//             );
//         }
        
//         *local = *local + 1;
//     }
// }

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
struct ImageMaterial {
    #[texture(101)]
    #[sampler(102)]
    image: Handle<Image>,
}

impl Material2d for ImageMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/image_shader.wgsl".into()
    }
}