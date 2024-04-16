use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_compute_noise::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin::<Worley2D>::default(),
            WorldInspectorPlugin::new(),
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}

fn _example(
    mut worley_noise_queue: ResMut<ComputeNoiseQueue<Worley2D>>,
    mut images: ResMut<Assets<Image>>,
) {
    let _worley_noise = worley_noise_queue.add(
        &mut images, 
        128, 128, 
        Worley2D::new((5, 5))
    );
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let worley_noise = ComputeNoiseQueue::<Worley2D>::create_image(&mut images, 128, 128);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(worley_noise.clone()),
            
            ..default()
        }),
        ..default()
    });

    commands.spawn(ComputeNoiseComponent::<Worley2D> {
        image: worley_noise.clone(),
        noise: Worley2D::new((5, 5)),
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
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

