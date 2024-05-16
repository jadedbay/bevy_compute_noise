use bevy::{prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}, texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};
use bevy_compute_noise::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<ImageMaterial>::default(),
            ComputeNoisePlugin::<Perlin2d>::default(),
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
) {
    let mut image = ComputeNoiseImage::create_image(ComputeNoiseSize::D2(1024, 1024));
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    let handle = images.add(image);

    let mut quad = Rectangle::default().mesh();
    if let Some(uvs) = quad.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        if let VertexAttributeValues::Float32x2(uvs) = uvs {
            for uv in uvs.iter_mut() {
                *uv = [uv[0] * 2.0, uv[1] * 2.0];
            }
        }
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(quad).into(),
            transform: Transform::default().with_scale(Vec3::splat(512.)),
            material: materials.add(ImageMaterial {
                image: handle.clone(),
            }),
            ..default()
        },
        ComputeNoiseComponent::<Perlin2d> {
            image: handle.clone(),
            noise: Perlin2d::new(1, 5),
        },
    ));

    commands.spawn(Camera2dBundle::default());
}

#[derive(Asset, AsBindGroup, Debug, Clone, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
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