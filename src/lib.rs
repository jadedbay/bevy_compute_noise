use std::marker::PhantomData;

use bevy::{
    asset::embedded_asset, prelude::*, render::{render_resource::SpecializedComputePipelines, Render, RenderApp, RenderSet}
};
use noise::{Perlin, Worley};
use noise_queue::{prepare_compute_noise_buffers, ComputeNoiseBufferQueue};
use render::{compute::{compute_noise, submit_compute_noise, ComputeNoiseEncoder}, pipeline::{load_fbm_shaders, load_compute_noise_shader, ComputeNoisePipelines}, prepare::prepare_compute_noise_pipelines};

use crate::{
    noise::ComputeNoiseType,
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
    render::{
        extract::extract_compute_noise_queue,
        prepare::prepare_bind_groups,
    },
};

pub mod image;
pub mod noise;
pub mod noise_queue;
mod render;

pub mod prelude {
    pub use crate::{
        image::{ComputeNoiseImage, ComputeNoiseSize},
        noise::{Worley, Perlin, PerlinFlags, WorleyFlags, Fbm},
        noise_queue::ComputeNoiseQueue,
        ComputeNoisePlugin
    };
}

#[derive(Default)]
pub struct ComputeNoiseTypePlugin<T: ComputeNoiseType>(PhantomData<T>);

impl<T: ComputeNoiseType> Plugin for ComputeNoiseTypePlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_shaders(app);
        app.register_type::<T>();
   }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        load_compute_noise_shader::<T>(render_app.world_mut());
        load_fbm_shaders::<T>(render_app.world_mut());
    }
}

// TODO: take in shader so can use custom noise with fbm
// pub struct ComputeNoisePlugin {
//     fbm_shader: &'static str,
// }
pub struct ComputeNoisePlugin;
impl Plugin for ComputeNoisePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "noise/shaders/util.wgsl");
        embedded_asset!(app, "noise/shaders/fbm.wgsl");

        app
            .add_plugins((
                ComputeNoiseTypePlugin::<Perlin>::default(),
                ComputeNoiseTypePlugin::<Worley>::default(),
            ))
            .init_resource::<ComputeNoiseQueue>()
            .init_resource::<ComputeNoiseBufferQueue>()
            .add_systems(PostUpdate, prepare_compute_noise_buffers);

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<ComputeNoiseRenderQueue>()
            .add_systems(ExtractSchedule, extract_compute_noise_queue)
            .add_systems(
                Render,
                (
                    prepare_compute_noise_pipelines.in_set(RenderSet::Prepare),
                    prepare_bind_groups.in_set(RenderSet::PrepareBindGroups),
                    (compute_noise, submit_compute_noise).after(RenderSet::PrepareBindGroups).before(RenderSet::Render).chain(),
                )
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ComputeNoisePipelines>()
            .init_resource::<SpecializedComputePipelines<ComputeNoisePipelines>>()
            .init_resource::<ComputeNoiseEncoder>();
    }
}