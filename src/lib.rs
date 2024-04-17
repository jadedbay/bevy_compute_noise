use std::marker::PhantomData;

use bevy::{prelude::*, render::{Render, RenderApp, RenderSet}};
use compute_noise::{update_noise, ComputeNoiseComponent};

use crate::{
    compute::{run_compute_noise, ComputeNoiseEncoder},
    compute_noise::ComputeNoise,
    extract::extract_compute_noise_queue,
    pipeline::ComputeNoisePipeline,
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
    prepare::prepare_bind_groups,
};

pub mod compute_noise;
pub mod noise_queue;
mod extract;
mod pipeline;
mod prepare;
mod compute;
pub mod image;

pub mod prelude {
    pub use crate::ComputeNoisePlugin;
    pub use crate::noise_queue::ComputeNoiseQueue;
    pub use crate::compute_noise::{ComputeNoiseComponent, Worley2D};
    pub use crate::image::ComputeNoiseImage;
}

#[derive(Default)]
pub struct ComputeNoisePlugin<T: ComputeNoise>(PhantomData<T>);

impl<T: ComputeNoise> Plugin for ComputeNoisePlugin<T> {
    fn build(&self, app: &mut App) {
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
                    run_compute_noise::<T>.in_set(RenderSet::Render),
                )
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);    
        render_app
            .init_resource::<ComputeNoisePipeline<T>>()
            .init_resource::<ComputeNoiseEncoder>();
    }
}