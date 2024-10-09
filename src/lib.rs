use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSet},
};
use render::compute::{compute_noise, submit_compute_noise, ComputeNoiseEncoder};

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
pub struct ComputeNoisePlugin<T: ComputeNoise>(PhantomData<T>);

impl<T: ComputeNoise> Plugin for ComputeNoisePlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_asset(app);

        app
            .register_type::<T>()
            .register_type::<ComputeNoiseComponent<T>>()
            .init_resource::<ComputeNoiseQueue<T>>()
            .add_systems(Update, update_noise::<T>);

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseRenderQueue<T>>()
            .add_systems(ExtractSchedule, extract_compute_noise_queue::<T>)
            .configure_sets(Render, ComputeNoiseSet.after(RenderSet::PrepareBindGroups))
            .add_systems(
                Render,
                (
                    prepare_bind_groups::<T>.in_set(RenderSet::PrepareBindGroups),
                    compute_noise::<T>.in_set(ComputeNoiseSet),
                    submit_compute_noise.after(ComputeNoiseSet),
                ),
            );
   }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ComputeNoisePipeline<T>>()
            .init_resource::<ComputeNoiseEncoder>();
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ComputeNoiseSet;