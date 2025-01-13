use bevy::prelude::*;
use bevy_compute_noise::prelude::*;

// This is a simple example to show how to queue a noise to be generated in a compute shader and written to a texture,
// if you want to see the result look at the test examples

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ComputeNoisePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut images: ResMut<Assets<Image>>,
    mut noise_queue: ResMut<ComputeNoiseQueue>,
) {
    let image = images.add(ComputeNoiseImage::create_image(ComputeNoiseSize::D2(512, 512)));
    noise_queue.generate(
        image,
        Perlin::default().into()
    );
}