use bevy::{image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}, prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}, renderer::RenderDevice}, sprite::{Material2d, Material2dPlugin}};
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

    noise_queue.add(
        handle.clone(), 
        ComputeNoiseBuilder::new()
            // .push_noise(Worley2d::new(1, 4, true))
            .push_noise(Perlin2d {
                seed: 5,
                frequency: 5,
                octaves: 4,
                persistence: 0.4,
                ..default()
            })
            .build(),
    );

    noise_queue.add(
        handle.clone(),
        Perlin2d {
            seed: 5,
            frequency: 5,
            octaves: 4,
            persistence: 1.0,
            ..default()
        }.into(),
    );

    commands.spawn((
        Mesh2d(meshes.add(quad)),
        Transform::default().with_scale(Vec3::splat(512.)),
        MeshMaterial2d(materials.add(ImageMaterial {
            image: handle.clone(),
        })),
        // ComputeNoiseComponent::<Perlin2d> {
        //     image: handle.clone(),
        //     noise: Perlin2d::new(0, 5, 4, true),
        // },
    ));

    commands.spawn(Camera2d::default());
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
