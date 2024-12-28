use bevy::{core_pipeline::tonemapping::Tonemapping, image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor}, prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}, window::PresentMode};
use bevy_compute_noise::prelude::*;
use iyes_perf_ui::{prelude::PerfUiDefaultEntries, PerfUiPlugin};


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                },
            ),
            Material2dPlugin::<ImageMaterial>::default(),
            ComputeNoisePlugin,
            PerfUiPlugin,
        ))
        .add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_noise)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut noise_queue: ResMut<ComputeNoiseQueue>
) {
    let mut image = ComputeNoiseImage::create_image(ComputeNoiseSize::D2(1024, 1024), ComputeNoiseFormat::Rgba);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    let handle = images.add(image);


    let mut quad = Rectangle::default().mesh().build();
    // if let Some(uvs) = quad.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
    //     if let VertexAttributeValues::Float32x2(uvs) = uvs {
    //         for uv in uvs.iter_mut() {
    //             *uv = [uv[0] * 2.0, uv[1] * 2.0];
    //         }
    //     }
    // }

    // noise_queue.add(
    //     handle.clone(),
    //     Perlin2d {
    //         seed: 1,
    //         frequency: 5,
    //         ..default()
    //     }.into(),
    // );

    noise_queue.add(
        handle.clone(),
        Fbm::<Perlin2d> {
            noise: Perlin2d {
                seed: 5,
                frequency: 5,
                // flags: (WorleyFlags::INVERT | WorleyFlags::TILEABLE).bits(),
                flags: (Perlin2dFlags::default() | Perlin2dFlags::TILEABLE).bits()
            },
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
        }.into(),
    );

    commands.spawn((
        Mesh2d(meshes.add(quad)),
        Transform::default().with_scale(Vec3::splat(512.)),
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(handle.clone()),
            ..default()
        }))
    ));

    commands.spawn((Camera2d, Tonemapping::None));
    commands.spawn(PerfUiDefaultEntries::default());
}

fn update_noise(
    mut noise_queue: ResMut<ComputeNoiseQueue>,
    query: Query<&MeshMaterial2d<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut local: Local<u32>,
    materials: Res<Assets<ColorMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        
        *local = *local + 1;
    }
    for material in query.iter() {
        for _ in 0..100 {
            noise_queue.add(
                materials.get(&material.0).unwrap().texture.as_ref().unwrap().clone(),
                    Fbm::<Perlin2d> {
                    noise: Perlin2d {
                        seed: 5,
                        frequency: 5,
                        // flags: (WorleyFlags::INVERT | WorleyFlags::TILEABLE).bits(),
                        flags: (Perlin2dFlags::default() | Perlin2dFlags::TILEABLE).bits()
                    },
                    octaves: 4,
                    lacunarity: 2.0,
                    persistence: 0.5,
                }.into(),
            );
        }
    }
}

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