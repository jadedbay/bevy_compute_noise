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
    worley_noise_queue.add(&mut images, 128, 128, Worley2DSettings::new(7));
}

fn setup(
    mut commands: Commands,
    mut worley_noise_queue: ResMut<ComputeNoiseQueue<Worley2D>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let worley_noise = worley_noise_queue.add(&mut images, 128, 128, Worley2DSettings::new(7));

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

