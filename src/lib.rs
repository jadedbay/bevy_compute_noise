use std::marker::PhantomData;

use bevy::{
    asset::embedded_asset, prelude::*, render::{Render, RenderApp, RenderSet}
};
use noise::{Perlin2d, Worley2d, Worley3d};
use noise_queue::{prepare_compute_noise_buffers, ComputeNoiseBufferQueue};
use render::{compute::{compute_noise, submit_compute_noise, ComputeNoiseEncoder}, pipeline::{ComputeNoiseFbmPipeline, ComputeNoisePipelines}};

use crate::{
    noise::ComputeNoise,
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
    render::{
        extract::extract_compute_noise_queue,
        pipeline::ComputeNoiseTypePipeline,
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
        ComputeNoiseTypePipeline::<T>::create_pipeline(render_app.world_mut());
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
                ComputeNoiseTypePlugin::<Perlin2d>::default(),
                ComputeNoiseTypePlugin::<Worley2d>::default(),
                ComputeNoiseTypePlugin::<Worley3d>::default(),
            ))
            .init_resource::<ComputeNoiseQueue>()
            .init_resource::<ComputeNoiseBufferQueue>()
            .add_systems(PostUpdate, prepare_compute_noise_buffers
        );

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
            .init_resource::<ComputeNoiseFbmPipeline>()
            .init_resource::<ComputeNoiseEncoder>();
    }
}