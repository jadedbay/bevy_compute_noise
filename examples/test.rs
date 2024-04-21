use std::f32::consts::PI;

use bevy::{prelude::*, render::{mesh::VertexAttributeValues, texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor}}};
use bevy_compute_noise::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin::<Worley2d>::default(),
            WorldInspectorPlugin::new(),
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let worley_noise = ComputeNoiseImage::create_image(
        &mut images, 
        ComputeNoiseSize::D2(128, 128)
    );

    let image = images.get_mut(worley_noise.clone()).unwrap();
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..default()
    });

    let mut plane = Mesh::from(Plane3d::default().mesh().size(5.0, 5.0));
    if let Some(uvs) = plane.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        if let VertexAttributeValues::Float32x2(uvs) = uvs {
            for uv in uvs.iter_mut() {
                *uv = [uv[0] * 2.0, uv[1] * 2.0];
            }
        }
    }
    commands.spawn(PbrBundle {
        mesh: meshes.add(plane),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(worley_noise.clone()),
            reflectance: 0.0,
            ..default()
        }),
        ..default()
    });

    commands.spawn(ComputeNoiseComponent::<Worley2d> {
        image: worley_noise.clone(),
        noise: Worley2d::new(1, 5),
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 500.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI * -0.15,
            PI * -0.15,
        )),
        ..default()
    });
}

