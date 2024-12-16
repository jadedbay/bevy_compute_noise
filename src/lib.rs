use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSet},
};
use noise::{Perlin2d, Worley2d, Worley3d};
use render::{compute::{compute_noise, submit_compute_noise, ComputeNoiseEncoder}, pipeline::ComputeNoisePipelines};

use crate::{
    noise::ComputeNoise,
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
        noise::{Worley2d, Worley3d, Perlin2d, ComputeNoiseBuilder},
        noise_queue::ComputeNoiseQueue,
        ComputeNoisePlugin
    };
}

#[derive(Default)]
pub struct ComputeNoiseTypePlugin<T: ComputeNoise>(PhantomData<T>);

impl<T: ComputeNoise> Plugin for ComputeNoiseTypePlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_shader(app);
        app.register_type::<T>();
   }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        ComputeNoisePipeline::<T>::create_pipeline(render_app.world_mut());
    }
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
            .init_resource::<ComputeNoiseQueue>();

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseRenderQueue>()
            .add_systems(ExtractSchedule, extract_compute_noise_queue)
            .add_systems(
                Render,
                (
                    prepare_bind_groups.in_set(RenderSet::PrepareBindGroups),
                    (compute_noise, submit_compute_noise).after(RenderSet::PrepareBindGroups).before(RenderSet::Render).chain(),
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