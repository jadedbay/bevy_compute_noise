use bevy::prelude::*;
use bevy_compute_noise::{compute_noise::worley_2d::Worley2DSettings, prelude::*};
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin::<Worley2D>::default(),
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}

fn example(
    mut worley_noise_queue: ResMut<ComputeNoiseQueue<Worley2D>>,
    mut images: ResMut<Assets<Image>>,
) {
    let worley_noise = worley_noise_queue.add(
        &mut images, 
        128, 128, 
        Worley2DSettings::new(7)
    );
}

fn setup(
    mut commands: Commands,
    mut worley_noise_queue: ResMut<ComputeNoiseQueue<Worley2D>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let worley_noise = worley_noise_queue.add(
        &mut images, 
        128, 128, 
        Worley2DSettings::new(2)
    );

    let test = worley_noise_queue.add_rgba::<Worley2D, Worley2D, Worley2D, Worley2D>(
        &mut images,
        128,
        128,
        (
            Some(Worley2DSettings::new(2)),
            Some(Worley2DSettings::new(2)),
            Some(Worley2DSettings::new(2)),
            Some(Worley2DSettings::new(2)),
        )
    );

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(worley_noise),
            
            ..default()
        }),
        ..default()
    });
}

