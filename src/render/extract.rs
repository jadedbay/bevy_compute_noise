use bevy::{prelude::*, render::MainWorld};

use crate::noise_queue::ComputeNoiseBufferQueue;

pub(crate) fn extract_compute_noise_queue(
    mut render_commands: Commands,
    mut world: ResMut<MainWorld>,
) {
    let mut main_compute_noise_queue = world.resource_mut::<ComputeNoiseBufferQueue>();
    render_commands.insert_resource(main_compute_noise_queue.clone());
    main_compute_noise_queue.queue.clear();
}