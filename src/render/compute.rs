use bevy::{prelude::*, render::{render_resource::{CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, PipelineCache}, renderer::{RenderDevice, RenderQueue}}};

use crate::noise_queue::ComputeNoiseRenderQueue;

#[derive(Resource)]
pub struct ComputeNoiseEncoder {
    encoder: Option<CommandEncoder>,
    submit: bool,
}
impl FromWorld for ComputeNoiseEncoder {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        ComputeNoiseEncoder {
            encoder: Some(render_device.create_command_encoder(
                &CommandEncoderDescriptor { 
                    label: Some("compute_noise_encoder"),
                }
            )),
            submit: false,
        }
    }
}

pub fn compute_noise(
    mut compute_noise_encoder: ResMut<ComputeNoiseEncoder>,
    mut compute_noise_queue: ResMut<ComputeNoiseRenderQueue>,
    mut pipeline_cache: ResMut<PipelineCache>,
) {
    if compute_noise_queue.queue.is_empty() { return; }

    pipeline_cache.process_queue();

    let Some(encoder) = &mut compute_noise_encoder.encoder else { return error!("Encoder is None") };
    let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
    
    let mut dispatched = Vec::new();
    for (seq_idx, sequence) in compute_noise_queue.queue.iter().enumerate() {
        if sequence.iter().all(|render_noise| {
            pipeline_cache.get_compute_pipeline(render_noise.pipeline_id).is_some()
        }) {
            for render_noise in sequence {
                let pipeline = pipeline_cache.get_compute_pipeline(render_noise.pipeline_id).unwrap();
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &render_noise.bind_group, &[]);
    
                let workgroups = render_noise.size.workgroup_count();
                pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
            }
            dispatched.push(seq_idx);

        }
    }

    for &i in dispatched.iter().rev() {
        compute_noise_queue.queue.remove(i);
    }

    if !dispatched.is_empty() {
        compute_noise_encoder.submit = true;
    }
}

pub fn submit_compute_noise(
    mut compute_noise_encoder: ResMut<ComputeNoiseEncoder>,
    render_queue: Res<RenderQueue>,
    render_device: Res<RenderDevice>
) {
    if compute_noise_encoder.submit {
        let encoder = compute_noise_encoder.encoder.take().unwrap();
        render_queue.submit(Some(encoder.finish()));
        compute_noise_encoder.encoder = Some(render_device.create_command_encoder(&CommandEncoderDescriptor { 
            label: Some("compute noise encoder") 
        }));
        compute_noise_encoder.submit = false;
    }
}