use bevy::{prelude::*, render::MainWorld};

use super::{ComputeNoiseReadbackSender, ComputeNoiseReadback};

pub(crate) fn extract_readback_sender(
    mut readback_sender: ResMut<ComputeNoiseReadbackSender>,
    mut world: ResMut<MainWorld>,
) {
    let mut main_readback = world.resource_mut::<ComputeNoiseReadback>();
    for handle in main_readback.queue.iter() {
        if let Some(sender) = main_readback.senders.get(handle) {
            readback_sender.0.insert(handle.clone(), sender.clone());
        }
    }

    main_readback.queue.clear();
}
