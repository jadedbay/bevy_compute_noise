use bevy::prelude::*;
use bevy_compute_noise::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoiseReadbackPlugin,
            ComputeNoisePlugin::<Worley2d>::default()
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut worley_2d_queue: ResMut<ComputeNoiseQueue<Worley2d>>,
    mut readback: ResMut<ComputeNoiseReadback>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = worley_2d_queue.add(&mut images, ComputeNoiseSize::D2(4, 4), Worley2d::new(1, 5));
    readback.queue(&mut images, image)
}
