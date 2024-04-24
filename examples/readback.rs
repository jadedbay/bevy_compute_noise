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
        .add_systems(Update, receive)
        .run();
}

fn setup(
    mut worley_2d_queue: ResMut<ComputeNoiseQueue<Worley2d>>,
    mut readback: ResMut<ComputeNoiseReadback>,
    mut images: ResMut<Assets<Image>>,
) {
    worley_2d_queue.add(&mut images, ComputeNoiseSize::D2(4, 4), Worley2d::new(1, 5), Some(&mut readback));
}

fn receive(
    input: Res<ButtonInput<KeyCode>>,
    readback_receiver: Res<ComputeNoiseReadbackReceiver>,
) {
    if input.just_pressed(KeyCode::Space) {
        for image in readback_receiver.images.iter() {
            if let Ok(data) = image.1.try_recv() {
                println!("Received data from render world: {data:?}");
            }
        }
    }
}