use bevy::{prelude::*, render::{render_resource::{CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, PipelineCache}, renderer::{RenderDevice, RenderQueue}}};

use crate::{noise::ComputeNoise, noise_queue::{CNRenderQueue, ComputeNoiseRenderQueue}};

use super::pipeline::{ComputeNoisePipeline, ComputeNoisePipelines};

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

// pub fn compute_noise<T: ComputeNoise>(
//     mut compute_noise_encoder: ResMut<ComputeNoiseEncoder>,
//     mut compute_noise_queue: ResMut<ComputeNoiseRenderQueue<T>>,
//     pipeline_cache: Res<PipelineCache>,
//     noise_pipeline: Res<ComputeNoisePipeline<T>>,
// ) {
//     if compute_noise_queue.queue.is_empty() { return; }

//     let mut dispatched = false;

//         if let Some(pipeline) = pipeline_cache.get_compute_pipeline(noise_pipeline.pipeline_id) {
//         {   
//             let Some(encoder) = &mut compute_noise_encoder.encoder else { return error!("Encoder is None") };
//             let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
//             pass.set_pipeline(pipeline);
            
//             for bind_groups in compute_noise_queue.queue.iter() {
//                 pass.set_bind_group(0, &bind_groups.image_bind_group, &[]);
//                 pass.set_bind_group(1, &bind_groups.noise_bind_group, &[]);
//                 let workgroups = bind_groups.size.workgroup_count();
//                 pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
                
//                 dispatched = true;
//             }
//         }

//         if dispatched {
//             compute_noise_encoder.submit = true;
//             compute_noise_queue.queue.clear() 
//         };
//     }
// }

pub fn compute_noise(
    mut compute_noise_encoder: ResMut<ComputeNoiseEncoder>,
    mut compute_noise_queue: ResMut<CNRenderQueue>,
    pipeline_cache: Res<PipelineCache>,
    pipelines: Res<ComputeNoisePipelines>,
) {
    if compute_noise_queue.queue.is_empty() { return; }
    let mut dispatched = false;

    let Some(encoder) = &mut compute_noise_encoder.encoder else { return error!("Encoder is None") };
    let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
    
    for bind_groups in compute_noise_queue.queue.iter() {
        pass.set_bind_group(0, &bind_groups.image_bind_group, &[]);
        
        for (noise_bind_group, type_id) in bind_groups.noise_bind_groups.iter() {
            if let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipelines.get_pipeline(*type_id).unwrap().pipeline_id) {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(1, noise_bind_group, &[]);
                
                let workgroups = bind_groups.size.workgroup_count();
                pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
                
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