use bevy::{prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}, texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}};
use bevy_compute_noise::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions};

fn main() {
    App::new()
        .register_asset_reflect::<Image3dMaterial>()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<Image3dMaterial>::default(),
            ComputeNoisePlugin::<Worley3d>::default(),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<Image3dMaterial>>,
    mut worley3d_queue: ResMut<ComputeNoiseQueue<Worley3d>>,
) {
    let mut image = ComputeNoiseImage::create_image(ComputeNoiseSize::D3(128, 128, 128), ComputeNoiseFormat::Rgba);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    let handle = images.add(image);
    
    worley3d_queue.add_image(&mut images, handle.clone(), Worley3d::new(1, 4, false));

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
            material: materials.add(Image3dMaterial {
                image: handle.clone(),
                layer: 0,
                texture_size: UVec3::new(128, 128, 128),
            }),
            ..default()
        },
    ));

    commands.spawn(Camera2dBundle::default());
}

#[derive(Asset, AsBindGroup, Debug, Clone, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
struct Image3dMaterial {
    #[texture(101, dimension = "3d")]
    #[sampler(102)]
    image: Handle<Image>,
    #[uniform(103)]
    layer: u32,
    #[uniform(104)]
    texture_size: UVec3,
}

impl Material2d for Image3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/image_3d_shader.wgsl".into()
    }
}