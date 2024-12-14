use bevy::{image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}, prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}, renderer::RenderDevice}, sprite::{Material2d, Material2dPlugin}};
use bevy_compute_noise::{noise::ComputeNoiseBuilder, noise_queue::CNQueue, prelude::*};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, quick::WorldInspectorPlugin, InspectorOptions};

fn main() {
    App::new()
        .register_asset_reflect::<Image3dMaterial>()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<Image3dMaterial>::default(),
            // ComputeNoisePlugin::<Worley3d>::default(),
            ComputeNoisePlugin,
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
    mut noise_queue: ResMut<CNQueue>,
    render_device: Res<RenderDevice>,
) {
    let mut image = ComputeNoiseImage::create_image(ComputeNoiseSize::D3(128, 128, 128), ComputeNoiseFormat::Rgba);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    let handle = images.add(image);

    noise_queue.add_image(
        &mut images, 
        handle.clone(), 
        ComputeNoiseBuilder::new().push_noise(Worley3d::new(1, 4, false)).build(),
        &render_device,
    );

    // let instructions = ComputeNoiseBuilder::new()
    //     .push_noise(Worley3d::new(1, 4, true))
    //     .push_noise(Perlin2d::new(0, 5, 4, false))
    //     .build();

    let mut quad = Rectangle::default().mesh().build();
    if let Some(uvs) = quad.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        if let VertexAttributeValues::Float32x2(uvs) = uvs {
            for uv in uvs.iter_mut() {
                *uv = [uv[0] * 2.0, uv[1] * 2.0];
            }
        }
    }

    commands.spawn(( 
        Mesh2d(meshes.add(quad).into()),
        MeshMaterial2d(materials.add(Image3dMaterial {
            image: handle.clone(),
            layer: 0,
        })),
        Transform::default().with_scale(Vec3::splat(512.)),
    ));

    commands.spawn(Camera2d::default());
}

#[derive(Asset, AsBindGroup, Debug, Clone, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
struct Image3dMaterial {
    #[texture(101, dimension = "3d")]
    #[sampler(102)]
    image: Handle<Image>,
    #[uniform(103)]
    layer: u32,
}

impl Material2d for Image3dMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/image_3d_shader.wgsl".into()
    }
}