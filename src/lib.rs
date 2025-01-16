use std::marker::PhantomData;

use bevy::{
    asset::embedded_asset, prelude::*, render::{render_resource::SpecializedComputePipelines, Render, RenderApp, RenderSet}
};
use crate::noise::modifiers::{invert::Invert, ComputeNoiseModifier};
use noise::generators::{Perlin, Worley};
use noise_queue::{prepare_compute_noise_buffers, ComputeNoiseBufferQueue};
use render::{compute::{compute_noise, submit_compute_noise, ComputeNoiseEncoder}, pipeline::{load_generator_shader, load_fbm_shaders, load_modifier_shader, ComputeNoisePipeline}};

use crate::{
    noise::generators::ComputeNoiseGenerator,
    noise_queue::{ComputeNoiseQueue, ComputeNoiseRenderQueue},
    render::{
        extract::extract_compute_noise_queue,
        prepare::prepare_render_noise,
    },
};

pub mod image;
pub mod noise;
pub mod noise_queue;
mod render;
mod shader;

pub mod prelude {
    pub use crate::{
        image::{ComputeNoiseImage, ComputeNoiseSize},
        noise::generators::{Worley, Perlin, PerlinFlags, WorleyFlags, Fbm},
        noise::modifiers::Invert,
        noise_queue::ComputeNoiseQueue,
        ComputeNoisePlugin
    };
}

#[derive(Default)]
pub struct ComputeNoiseGeneratorPlugin<T: ComputeNoiseGenerator>(PhantomData<T>);

impl<T: ComputeNoiseGenerator> Plugin for ComputeNoiseGeneratorPlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_shaders(app);
        app.register_type::<T>();
   }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        load_generator_shader::<T>(render_app.world_mut());
        load_fbm_shaders::<T>(render_app.world_mut());
    }
}

#[derive(Default)]
pub struct ComputeNoiseModificationPlugin<T: ComputeNoiseModifier>(PhantomData<T>);
impl<T: ComputeNoiseModifier> Plugin for ComputeNoiseModificationPlugin<T> {
    fn build(&self, app: &mut App) {
        T::embed_shaders(app);
    }

    fn finish(&self, app: &mut App) { 
        let render_app = app.sub_app_mut(RenderApp);
        load_modifier_shader::<T>(render_app.world_mut());
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
        embedded_asset!(app, "noise/generators/shaders/fbm.wgsl");

        app
            .add_plugins((
                ComputeNoiseGeneratorPlugin::<Perlin>::default(),
                ComputeNoiseGeneratorPlugin::<Worley>::default(),
                ComputeNoiseModificationPlugin::<Invert>::default(),
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
                    prepare_render_noise.in_set(RenderSet::PrepareBindGroups),
                    (compute_noise, submit_compute_noise).after(RenderSet::PrepareBindGroups).before(RenderSet::Render).chain(),
                )
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<ComputeNoisePipeline>()
            .init_resource::<SpecializedComputePipelines<ComputeNoisePipeline>>()
            .init_resource::<ComputeNoiseEncoder>();
    }
}