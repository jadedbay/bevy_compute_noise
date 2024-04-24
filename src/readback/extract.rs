use bevy::{prelude::*, render::MainWorld};

use super::ComputeNoiseReadbackSender;

pub(crate) fn extract_readback_sender(
    mut render_commands: Commands,
    mut world: ResMut<MainWorld>,
) {
    let mut main_readback_sender = world.resource_mut::<ComputeNoiseReadbackSender>();
    render_commands.insert_resource(main_readback_sender.clone());
    main_readback_sender.images.clear();
}