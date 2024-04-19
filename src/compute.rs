use bevy::{prelude::*, render::{render_resource::{CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, PipelineCache}, renderer::{RenderDevice, RenderQueue}}};

use crate::{compute_noise::ComputeNoise, noise_queue::ComputeNoiseRenderQueue, pipeline::ComputeNoisePipeline};

#[derive(Resource)]
pub struct ComputeNoiseEncoder(Option<CommandEncoder>);

impl FromWorld for ComputeNoiseEncoder {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        ComputeNoiseEncoder(Some(render_device.create_command_encoder(&CommandEncoderDescriptor { 
            label: Some("compute_noise_encoder") 
        })))
    }
}

pub fn run_compute_noise<T: ComputeNoise>(
    mut compute_noise_renderer: ResMut<ComputeNoiseEncoder>,
    mut compute_noise_queue: ResMut<ComputeNoiseRenderQueue<T>>,
    pipeline_cache: Res<PipelineCache>,
    worley_pipeline: Res<ComputeNoisePipeline<T>>,
    render_queue: Res<RenderQueue>,
    render_device: Res<RenderDevice>,
) {
    let mut dispatched = false;

    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(worley_pipeline.pipeline_id) {
        {   
            let Some(encoder) = &mut compute_noise_renderer.0 else { return error!("Encoder is None") };
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            pass.set_pipeline(pipeline);
            
            for (image_bind_group, noise_bind_group, size) in compute_noise_queue.queue.iter() {
                pass.set_bind_group(0, &image_bind_group, &[]);
                pass.set_bind_group(1, &noise_bind_group, &[]);

                let workgroups = size.workgroup_count();
                pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
                
                dispatched = true;
                dbg!("DISPATCHED");
            }
        }

        let encoder = compute_noise_renderer.0.take().unwrap();
        render_queue.submit(Some(encoder.finish()));

        compute_noise_renderer.0 = Some(render_device.create_command_encoder(&CommandEncoderDescriptor { 
            label: Some("compute noise encoder") 
        }));

        if dispatched { compute_noise_queue.queue.clear() };
    }
}