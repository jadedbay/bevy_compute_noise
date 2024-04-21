use std::marker::PhantomData;

use bevy::{prelude::*, render::{render_graph::RenderGraph, Render, RenderApp, RenderSet}};

use crate::{
    noise::{
        ComputeNoise,
        ComputeNoiseComponent,
        update_noise,
    },
    render::{
        extract::extract_compute_noise_queue,
        pipeline::ComputeNoisePipeline,
        prepare::prepare_bind_groups,
        node::ComputeNoiseNode,
    },
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
};

pub mod noise;
pub mod noise_queue;
mod render;
pub mod image;

pub mod prelude {
    pub use crate::{
        ComputeNoisePlugin,
        noise_queue::ComputeNoiseQueue,
        noise::{ComputeNoiseComponent, Worley2d, Worley3d},
        image::{ComputeNoiseImage, ComputeNoiseSize},
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

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseRenderQueue<T>>()
            .add_systems(ExtractSchedule, extract_compute_noise_queue::<T>)
            .add_systems(
                Render, 
                (
                    prepare_bind_groups::<T>.in_set(RenderSet::PrepareBindGroups),
                )
            );

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(T::render_label(), ComputeNoiseNode::<T>::default());
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);    
        render_app.init_resource::<ComputeNoisePipeline<T>>();
    }
}