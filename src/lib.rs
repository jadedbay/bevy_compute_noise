use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{render_graph::RenderGraphApp, Render, RenderApp, RenderSet},
};
use noise::{Perlin2d, Worley2d, Worley3d};
use noise_queue::{CNQueue, CNRenderQueue};
use render::{compute::{compute_noise, submit_compute_noise, ComputeNoiseEncoder}, pipeline::ComputeNoisePipelines, prepare::prepare_cn_bind_groups};

use crate::{
    noise::{ComputeNoise, ComputeNoiseComponent, update_noise},
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
    render::{
        extract::extract_compute_noise_queue,
        pipeline::ComputeNoisePipeline,
        prepare::prepare_bind_groups,
    },
};

pub mod image;
pub mod noise;
pub mod noise_queue;
mod render;

pub mod prelude {
    pub use crate::{
        image::{ComputeNoiseImage, ComputeNoiseSize, ComputeNoiseFormat},
        noise::{ComputeNoiseComponent, Worley2d, Worley3d, Perlin2d},
        noise_queue::ComputeNoiseQueue,
        ComputeNoisePlugin
    };
}

#[derive(Default)]
pub struct ComputeNoiseTypePlugin<T: ComputeNoise>(PhantomData<T>);

impl<T: ComputeNoise> Plugin for ComputeNoiseTypePlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_asset(app);

        app
            .register_type::<T>()
            .register_type::<ComputeNoiseComponent<T>>()
            .init_resource::<ComputeNoiseQueue<T>>()
            .add_systems(Update, update_noise::<T>);

        let render_app = app.sub_app_mut(RenderApp);

        // render_app
            // .init_resource::<ComputeNoiseRenderQueue<T>>()
            // .add_systems(ExtractSchedule, extract_compute_noise_queue::<T>)
            // .configure_sets(Render, ComputeNoiseSet.after(RenderSet::PrepareBindGroups))
            // .add_systems(
            //     Render,
            //     (
            //         prepare_bind_groups::<T>.in_set(RenderSet::PrepareBindGroups),
            //         compute_noise::<T>.in_set(ComputeNoiseSet),
            //         submit_compute_noise.after(ComputeNoiseSet),
            //     ),
            // );
   }

    // fn finish(&self, app: &mut App) {
    //     let render_app = app.sub_app_mut(RenderApp);
    //     render_app
            // .init_resource::<ComputeNoisePipeline<T>>();
            // .init_resource::<ComputeNoiseEncoder>();
    // }
}

pub struct ComputeNoisePlugin;
impl Plugin for ComputeNoisePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
            ComputeNoiseTypePlugin::<Perlin2d>::default(),
            ComputeNoiseTypePlugin::<Worley2d>::default(),
            ComputeNoiseTypePlugin::<Worley3d>::default(),
        ))
            .init_resource::<CNQueue>();

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<CNRenderQueue>()
            .add_systems(ExtractSchedule, extract_compute_noise_queue)
            .configure_sets(Render, ComputeNoiseSet.after(RenderSet::PrepareBindGroups))
            .add_systems(
                Render,
                (
                    prepare_cn_bind_groups.in_set(RenderSet::PrepareBindGroups),
                    (compute_noise, submit_compute_noise).in_set(ComputeNoiseSet).chain(),
                )
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ComputeNoisePipelines>()
            .init_resource::<ComputeNoiseEncoder>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ComputeNoiseSet;