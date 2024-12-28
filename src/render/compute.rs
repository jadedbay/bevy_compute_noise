use bevy::{prelude::*, render::{render_resource::{CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, LoadOp, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, StoreOp}, renderer::{RenderDevice, RenderQueue}}};

use crate::noise_queue::ComputeNoiseRenderQueue;

use super::pipeline::ComputeNoiseRenderPipeline;

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
    render_pipeline: Res<ComputeNoiseRenderPipeline>,
) {
    if compute_noise_queue.queue.is_empty() { return; }
    pipeline_cache.process_queue();

    let mut dispatched = false;

    let Some(encoder) = &mut compute_noise_encoder.encoder else { return error!("Encoder is None") };
    // let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
    
    for bind_groups in compute_noise_queue.queue.iter() {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("cn_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &bind_groups.texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::default(),
                    store: StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        // pass.set_bind_group(0, &bind_groups.image_bind_group, &[]);
        
        for (noise_bind_group, compute_noise_pipeline) in bind_groups.noise_bind_groups.iter() {
            // if let Some(pipeline) = pipeline_cache.get_compute_pipeline(compute_noise_pipeline.pipeline_id) {
            if let Some(pipeline) = pipeline_cache.get_render_pipeline(render_pipeline.pipeline_id) {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, noise_bind_group, &[]);

                pass.draw(0..3, 0..1);
                
                // let workgroups = bind_groups.size.workgroup_count();
                // pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
                
                dispatched = true;
            }
        }
    }

    if dispatched {
        compute_noise_encoder.submit = true;
        compute_noise_queue.queue.clear();
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