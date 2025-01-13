use bevy::{
    prelude::*, render::{
        render_resource::{BindGroup, Buffer, CachedComputePipelineId, TextureDimension}, renderer::RenderDevice,
    }
};

use crate::{image::ComputeNoiseSize, noise::ErasedComputeNoise, render::pipeline::{ComputeNoisePipelineKey, NoiseOp}};

pub struct ComputeNoiseInstruction {
    images: Vec<Handle<Image>>,
    noise: ErasedComputeNoise,
    op: NoiseOp,
}
pub struct ComputeNoiseSequence(Vec<ComputeNoiseInstruction>);

pub struct ComputeNoiseSequenceBuilder {
    instructions: Vec<ComputeNoiseInstruction>,
    image: Option<Handle<Image>>,
}

impl ComputeNoiseSequenceBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            image: None,
        }
    }

    pub fn generate(mut self, output: Handle<Image>, noise: ErasedComputeNoise) -> Self {
        self.instructions.push(ComputeNoiseInstruction {
            images: vec![output],
            noise,
            op: NoiseOp::Generator,
        });
        self
    }
    
    pub fn modify(mut self, input: Handle<Image>, output: Handle<Image>, modifier: ErasedComputeNoise) -> Self {
        self.instructions.push(ComputeNoiseInstruction {
            images: vec![input, output],
            noise: modifier,
            op: NoiseOp::Modifier,
        });
        self
    }
    
    pub fn combine(mut self, input1: Handle<Image>, input2: Handle<Image>, output: Handle<Image>, combiner: ErasedComputeNoise) -> Self {
        self.instructions.push(ComputeNoiseInstruction {
            images: vec![input1, input2, output],
            noise: combiner,
            op: NoiseOp::Combiner,
        });
        self
    }

    pub fn new_image(image: Handle<Image>) -> Self {
        Self {
            instructions: Vec::new(),
            image: Some(image),
        }
    }

    pub fn generate_image(self, noise: ErasedComputeNoise) -> Self {
        let image = self.image.clone().expect("No image set. Use new_image()");
        self.generate(image, noise)
    }

    pub fn modify_image(self, noise: ErasedComputeNoise) -> Self {
        let image = self.image.clone().expect("No image set. Use new_image()");
        self.modify(image.clone(), image, noise)
    }

    pub fn combine_image(self, other: Handle<Image>, noise: ErasedComputeNoise) -> Self {
        let image = self.image.clone().expect("No image set. Use new_image()");
        self.combine(image.clone(), other, image.clone(), noise)
    }
    
    pub fn build(self) -> ComputeNoiseSequence {
        ComputeNoiseSequence(self.instructions)
    }
}

// Main World
#[derive(Resource, Default)]
pub struct ComputeNoiseQueue {
    pub(crate) queue: Vec<ComputeNoiseSequence>,
}
impl ComputeNoiseQueue {
    pub fn generate(&mut self, output: Handle<Image>, noise: ErasedComputeNoise) {
        self.queue.push(
            ComputeNoiseSequence(vec![
                ComputeNoiseInstruction {
                    images: vec![output],
                    noise,
                    op: NoiseOp::Generator
                }
            ])
        );
    }

    pub fn modify(&mut self, input: Handle<Image>, output: Handle<Image>, modifier: ErasedComputeNoise) {
        self.queue.push(
            ComputeNoiseSequence(vec![
                ComputeNoiseInstruction {
                    images: vec![input, output],
                    noise: modifier,
                    op: NoiseOp::Modifier
                }
            ])
        );
    }

    pub fn combine(&mut self, input1: Handle<Image>, input2: Handle<Image>, output: Handle<Image>, combiner: ErasedComputeNoise) {
        self.queue.push(
            ComputeNoiseSequence(vec![
                ComputeNoiseInstruction {
                    images: vec![input1, input2, output],
                    noise: combiner,
                    op: NoiseOp::Combiner
                }
            ])
        );
    }

    pub fn sequence(&mut self, sequence: ComputeNoiseSequence) {
        self.queue.push(sequence);
    }
}

pub fn prepare_compute_noise_buffers(
    images: Res<Assets<Image>>,
    render_device: Res<RenderDevice>,
    mut noise_queue: ResMut<ComputeNoiseQueue>,
    mut noise_buffer_queue: ResMut<ComputeNoiseBufferQueue>,
) {
    for item in &noise_queue.queue {
        let sizes: Vec<ComputeNoiseSize> = item.0.iter()
            .flat_map(|instruction| instruction.images.iter())
            .map(|image_handle| {
                images.get(image_handle).unwrap().texture_descriptor.size.into()
            })
            .collect();

        if !sizes.windows(2).all(|window| TextureDimension::from(window[0]) == TextureDimension::from(window[1])) {
            error!("Not all images have the same dimension - did not queue compute noise.");
            continue;
        }

        let sequence_buffers: Vec<ComputeNoiseBuffers> = item.0.iter().zip(&sizes).map(|(instruction, size)| {
            ComputeNoiseBuffers {
                key: ComputeNoisePipelineKey {
                    type_id: instruction.noise.type_id,
                    dimension: (*size).into(),
                    op: instruction.op,
                },
                images: instruction.images.clone(),
                buffers: instruction.noise.buffers(&render_device),
                size: *size,
            }
        }).collect();

        noise_buffer_queue.queue.push(sequence_buffers);
    }
    
    noise_queue.queue.clear();
}

// Main/Render World
#[derive(Clone)]
pub struct ComputeNoiseBuffers {
    pub key: ComputeNoisePipelineKey,
    pub images: Vec<Handle<Image>>,
    pub buffers: Vec<Buffer>,
    pub size: ComputeNoiseSize,
}

#[derive(Resource, Clone, Default)]
pub struct ComputeNoiseBufferQueue {
    pub queue: Vec<Vec<ComputeNoiseBuffers>>,
}

// Render World
#[derive(Clone)]
pub struct RenderComputeNoise {
    pub key: ComputeNoisePipelineKey,
    pub bind_group: BindGroup,
    pub pipeline_id: CachedComputePipelineId,
    pub size: ComputeNoiseSize,
}

#[derive(Default, Resource)]
pub(crate) struct ComputeNoiseRenderQueue {
    pub queue: Vec<Vec<RenderComputeNoise>>,
}
