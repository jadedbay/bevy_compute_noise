use bevy::{prelude::*, render::MainWorld};

use super::ComputeNoiseReadbackSender;

pub(crate) fn extract_readback_sender(
    mut readback_sender: ResMut<ComputeNoiseReadbackSender>,
    mut world: ResMut<MainWorld>,
) {
    let mut main_readback_sender = world.resource_mut::<ComputeNoiseReadbackSender>();
    readback_sender.images.extend(main_readback_sender.images.iter().map(|(k, v)| (k.clone(), v.clone())));
    main_readback_sender.images.clear();
}