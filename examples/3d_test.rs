use bevy::prelude::*;
use bevy_compute_noise::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin::<Worley3d>::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut noise_queue: ResMut<ComputeNoiseQueue<Worley3d>>,
    mut images: ResMut<Assets<Image>>,
) {
    noise_queue.add(&mut images, ComputeNoiseSize::D3(128, 128, 128), Worley3d::new(1, 5));
}