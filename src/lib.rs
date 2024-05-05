use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{render_graph::RenderGraph, Render, RenderApp, RenderSet},
};

use noise::auto_readback_image;
use prelude::ComputeNoiseReadback;
use readback::{extract::extract_readback_sender, prepare::prepare_readback_buffers, read::map_read_texture, ComputeNoiseReadbackSender};

use crate::{
    noise::{update_noise, ComputeNoise, ComputeNoiseComponent},
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
    render::{
        extract::extract_compute_noise_queue,
        node::ComputeNoiseNode,
        pipeline::ComputeNoisePipeline,
        prepare::{clear_render_queue, prepare_bind_groups},
    },
};

pub mod image;
pub mod noise;
pub mod noise_queue;
mod render;
mod readback;

pub mod prelude {
    pub use crate::{
        image::{ComputeNoiseImage, ComputeNoiseSize},
        noise::{ComputeNoiseComponent, ComputeNoiseAutoReadback, Worley2d, Worley3d},
        noise_queue::ComputeNoiseQueue,
        readback::ComputeNoiseReadback,
        ComputeNoisePlugin,
        ComputeNoiseReadbackPlugin,
    };
}

#[derive(Default)]
pub struct ComputeNoisePlugin<T: ComputeNoise>(PhantomData<T>);

impl<T: ComputeNoise> Plugin for ComputeNoisePlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_asset(app);

        app
            .register_type::<ComputeNoiseComponent<T>>()
            .init_resource::<ComputeNoiseQueue<T>>()
            .add_systems(Update, update_noise::<T>);

        let readback = app.is_plugin_added::<ComputeNoiseReadbackPlugin>();

        if readback {
            app.add_systems(PreUpdate, auto_readback_image::<T>);
        }
        
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseRenderQueue<T>>()
            .add_systems(ExtractSchedule, extract_compute_noise_queue::<T>)
            .add_systems(
                Render,
                (
                    prepare_bind_groups::<T>.in_set(RenderSet::PrepareBindGroups),
                    clear_render_queue::<T>.in_set(RenderSet::Cleanup),
                ),
            );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(T::render_label(), ComputeNoiseNode::<T>::default());

        if readback {
            render_app.add_systems(Render, map_read_texture::<T>.after(RenderSet::Render).before(RenderSet::Cleanup));
        }
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ComputeNoisePipeline<T>>();
    }
}

// Add readback plugin before any noise plugins
pub struct ComputeNoiseReadbackPlugin;

impl Plugin for ComputeNoiseReadbackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ComputeNoiseReadback>();
    
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseReadbackSender>()
            .add_systems(ExtractSchedule, extract_readback_sender)
            .add_systems(Render, prepare_readback_buffers.in_set(RenderSet::PrepareResources));
    }
}

#[test]
fn test() {
    let base_cell = IVec3::new(-1, 5, -1);
    let cell_count = 5;
    let texture_size = Vec3::new(128., 128., 128.);

    let cell = (base_cell + cell_count) % cell_count;
    let cell_offset = (
        if cell.x != base_cell.x { base_cell.x.signum() as f32 * texture_size.x } else { 0.0 },
        if cell.y != base_cell.y { base_cell.y.signum() as f32 * texture_size.y } else { 0.0 },
        if cell.z != base_cell.z { base_cell.z.signum() as f32 * texture_size.z } else { 0.0 },
    );

    dbg!(cell_offset);
    dbg!(cell);
}