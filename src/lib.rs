use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{render_graph::RenderGraph, Render, RenderApp, RenderSet},
};

use noise::auto_readback_image;
use prelude::ComputeNoiseReadback;
use readback::{extract::extract_readback_sender, prepare::prepare_readback_buffers, read::readback_texture, ComputeNoiseReadbackSender};

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
            render_app.add_systems(Render, readback_texture::<T>.after(RenderSet::Render).before(RenderSet::Cleanup));
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
        app
            .init_resource::<ComputeNoiseReadback>()
            .add_systems(PreUpdate, auto_readback_image);
    
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseReadbackSender>()
            .add_systems(ExtractSchedule, extract_readback_sender)
            .add_systems(Render, prepare_readback_buffers.in_set(RenderSet::PrepareResources));
    }
}