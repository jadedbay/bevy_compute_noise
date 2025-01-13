use bevy::{image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}, prelude::*, render::{mesh::VertexAttributeValues, render_resource::{AsBindGroup, ShaderRef}}, sprite::{Material2d, Material2dPlugin}};
use bevy_compute_noise::prelude::*;

fn main() {
    App::new()
        .register_asset_reflect::<Image3dMaterial>()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<Image3dMaterial>::default(),
            ComputeNoisePlugin,
        ))
        .add_systems(Startup, setup)
        // .add_systems(Update, update_noise)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<Image3dMaterial>>,
    mut noise_queue: ResMut<ComputeNoiseQueue>,
) {
    let mut image = ComputeNoiseImage::create_image(ComputeNoiseSize::D3(128, 128, 128));
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });
    let handle = images.add(image);

    noise_queue.queue(
        handle.clone(),
        Fbm::<Perlin> {
            noise: Perlin {
                seed: 1,
                frequency: 5.0,
                // flags: (WorleyFlags::INVERT | WorleyFlags::TILEABLE).bits()
                flags: (PerlinFlags::default() | PerlinFlags::TILEABLE).bits()
            },
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
        },
    );

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

// fn update_noise(
//     mut noise_queue: ResMut<ComputeNoiseQueue>,
//     query: Query<&MeshMaterial2d<Image3dMaterial>>,
//     keys: Res<ButtonInput<KeyCode>>,
//     mut local: Local<u32>,
//     materials: Res<Assets<Image3dMaterial>>,
// ) {
//     if keys.just_pressed(KeyCode::Space) {
//         for material in query.iter() {
//             noise_queue.generate(
//                 materials.get(&material.0).unwrap().image.clone(),
//                 Fbm::<Perlin> {
//                     noise: Perlin {
//                         seed: *local,
//                         frequency: 5.0,
//                         // flags: (WorleyFlags::INVERT | WorleyFlags::TILEABLE).bits(),
//                         flags: (PerlinFlags::default() | PerlinFlags::TILEABLE).bits()
//                     },
//                     octaves: 4,
//                     lacunarity: 2.0,
//                     persistence: 0.5,
//                 }.into(),
//             );
//         }
        
//         *local = *local + 1;
//     }
// }

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
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
