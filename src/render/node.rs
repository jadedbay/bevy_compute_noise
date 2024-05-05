use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets, render_graph::{self, NodeRunError, RenderGraphContext}, render_resource::{CachedPipelineState, ComputePassDescriptor, Extent3d, ImageCopyBuffer, PipelineCache}, renderer::RenderContext
    },
};

use crate::{
    noise::ComputeNoise, noise_queue::ComputeNoiseRenderQueue, readback::{util::layout_data, ComputeNoiseReadbackSender}, render::pipeline::ComputeNoisePipeline
};

#[derive(Default, Clone, Copy)]
pub(crate) enum ComputeNoiseNodeState {
    #[default]
    Loading,
    Ready,
}

#[derive(Default)]
pub struct ComputeNoiseNode<T: ComputeNoise> {
    state: ComputeNoiseNodeState,
    _phantom_data: PhantomData<T>,
}

impl<T: ComputeNoise> ComputeNoiseNode<T> {
    pub fn get_state(&self) -> ComputeNoiseNodeState {
        self.state
    }
}

impl<T: ComputeNoise> render_graph::Node for ComputeNoiseNode<T> {
    fn update(&mut self, world: &mut World) {
        match self.state {
            ComputeNoiseNodeState::Loading => {
                let pipeline = world.resource::<ComputeNoisePipeline<T>>();
                let pipeline_cache = world.resource::<PipelineCache>();
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.pipeline_id)
                {
                    self.state = ComputeNoiseNodeState::Ready;
                }
            }
            ComputeNoiseNodeState::Ready => {}
        }
    }

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        match self.state {
            ComputeNoiseNodeState::Loading => {}
            ComputeNoiseNodeState::Ready => {
                let compute_noise_queue = world.resource::<ComputeNoiseRenderQueue<T>>();
                let pipeline_id = world.resource::<ComputeNoisePipeline<T>>();
                let pipeline_cache = world.resource::<PipelineCache>();

                let readback = world.get_resource::<ComputeNoiseReadbackSender>();

                let pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline_id.pipeline_id)
                    .unwrap();

                let mut readback_handles = Vec::new();

                {
                    let mut pass = render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor::default());

                    pass.set_pipeline(pipeline);
                    for bind_groups in compute_noise_queue.queue.iter() {
                        pass.set_bind_group(0, &bind_groups.image_bind_group, &[]);
                        pass.set_bind_group(1, &bind_groups.noise_bind_group, &[]);

                        let workgroups = bind_groups.size.workgroup_count();
                        pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);

                        if let Some(readback) = readback {
                            if readback.0.contains_key(&bind_groups.handle) {
                                readback_handles.push(bind_groups.handle.clone());
                            }
                        }
                    }
                }

                if let Some(readback) = readback {
                    for handle in readback_handles {
                        let images = world.resource::<RenderAssets<Image>>();
                        let image = images.get(&handle).unwrap();
                        let size = readback.0.get(&handle).unwrap().size;
                        render_context.command_encoder().copy_texture_to_buffer(
                            image.texture.as_image_copy(), 
                            ImageCopyBuffer {
                                buffer: &readback.0.get(&handle).unwrap().buffer.as_ref().unwrap(),
                                layout: layout_data(size.width(), size.height(), image.texture_format)
                            },
                            Extent3d {
                                width: image.size.x as u32,
                                height: image.size.y as u32,
                                ..default()
                            }
                        );
                    }
                }
            }
        }

        Ok(())
    }
}