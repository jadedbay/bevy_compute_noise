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
        noise::{ComputeNoiseComponent, ComputeNoiseAutoReadback, Worley2d, Worley3d, Perlin2d},
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

// Add readback plugin before any noise plugins <- maybe? i dont remember why i added this comment
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
    let frequency: u32 = 3;
    
    let texture_size = Vec2::new(128., 128.);
    let cell_size = texture_size / frequency as f32;

    let pixel = UVec2::new(21, 21);
    let u_base = Vec2::new(pixel.x as f32 / cell_size.x, pixel.y as f32 / cell_size.y);
    let base = UVec2::new(u_base.x as u32, u_base.y as u32);
    let cell = (base + frequency) % frequency;

    let location_in_cell_x = (pixel.x as f32 - (cell_size.x * cell.x as f32)) / cell_size.x;

    dbg!(cell_size);
    dbg!(location_in_cell_x);
}