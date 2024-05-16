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
    mut commands: Commands,
    mut worley_2d_queue: ResMut<ComputeNoiseQueue<Worley2d>>,
    mut readback: ResMut<ComputeNoiseReadback>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = worley_2d_queue.add(&mut images, ComputeNoiseSize::D2(4, 4), Worley2d::new(1, 5));
    readback.queue(&mut images, image.clone());

    commands.insert_resource(NoiseImage(image));
}

fn receive(
    mut images: ResMut<Assets<Image>>,
    image: Res<NoiseImage>,
    readback: Res<ComputeNoiseReadback>,
) {
    if readback.receive(&mut images, image.0.clone()).is_some() {
        dbg!("RECEIVED");
    } else {
        dbg!("NOT RECEIVED");
    }
}

#[derive(Resource)]
struct NoiseImage(Handle<Image>);