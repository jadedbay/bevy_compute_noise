use bevy::{prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}, texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}}};
use bevy_compute_noise::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions};

fn main() {
    App::new()
        .register_asset_reflect::<Image3dMaterial>()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<Image3dMaterial>::default(),
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
    let worley_noise = ComputeNoiseImage::create_image(&mut images, ComputeNoiseSize::D3(128, 128, 128));
    let image = images.get_mut(worley_noise.clone()).unwrap();
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    
    worley3d_queue.add_image(&mut images, worley_noise.clone(), Worley3d::new(1, 4));

    let mut plane = Mesh::from(Plane3d::default().mesh().size(5.0, 5.0));
    if let Some(uvs) = plane.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        if let VertexAttributeValues::Float32x2(uvs) = uvs {
            for uv in uvs.iter_mut() {
                *uv = [uv[0] * 2.0, uv[1] * 2.0];
            }
        }
    }
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(plane),
        material: materials.add(Image3dMaterial {
            image: worley_noise.clone(),
            layer: 0,
            texture_size: UVec3::new(128, 128, 128),
        }),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

impl Material for Image3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/image_3d_shader.wgsl".into()
    }
}