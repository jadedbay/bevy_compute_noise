use std::marker::PhantomData;

use bevy::{
    prelude::*,
    render::{
        render_graph::{self, NodeRunError, RenderGraphContext}, render_resource::{CachedPipelineState, ComputePassDescriptor, PipelineCache}, renderer::RenderContext
    },
};

use crate::{
    noise::ComputeNoise, noise_queue::ComputeNoiseRenderQueue, render::pipeline::ComputeNoisePipeline
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

                let pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline_id.pipeline_id)
                    .unwrap();

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
                    }
                }
            }
        }

        Ok(())
    }
}