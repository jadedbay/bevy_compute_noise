use bevy::prelude::*;
use bevy_compute_noise::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin::<Worley2d>::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut worley2d_queue: ResMut<ComputeNoiseQueue<Worley2d>>,
    mut images: ResMut<Assets<Image>>,
) {
    worley2d_queue.add(&mut images, ComputeNoiseSize::D2(128, 128), Worley2d::new(1, 5));
}