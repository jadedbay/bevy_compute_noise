use bevy::{prelude::*, render::MainWorld};

use crate::{noise::ComputeNoise, noise_queue::ComputeNoiseQueue};

pub(crate) fn extract_compute_noise_queue<T: ComputeNoise>(
    mut render_commands: Commands,
    mut world: ResMut<MainWorld>,
) {
    let mut main_compute_noise_queue = world.resource_mut::<ComputeNoiseQueue<T>>();
    render_commands.insert_resource(main_compute_noise_queue.clone());
    main_compute_noise_queue.clear();
}