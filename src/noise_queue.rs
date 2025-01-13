use bevy::{
    prelude::*, render::{
        render_resource::{BindGroup, Buffer, CachedComputePipelineId, TextureDimension}, renderer::RenderDevice,
    }
};

use crate::{image::ComputeNoiseSize, noise::{ComputeNoise, ErasedComputeNoise}, render::pipeline::{ComputeNoisePipelineKey, NoiseOp}};

pub struct ComputeNoiseInstruction {
    images: Vec<Handle<Image>>,
    noise: ErasedComputeNoise,
    op: NoiseOp,
}
pub struct ComputeNoiseSequence(Vec<ComputeNoiseInstruction>);

pub enum QueueNoiseOp {
    Generate(ErasedComputeNoise),
    Modify(Handle<Image>, ErasedComputeNoise),
    Combine(Handle<Image>, Handle<Image>, ErasedComputeNoise),
}

impl QueueNoiseOp {
    fn resolve_images(self, output: Handle<Image>) -> Self {
        match self {
            QueueNoiseOp::Generate(noise) => QueueNoiseOp::Generate(noise),
            QueueNoiseOp::Modify(input, noise) if input == Handle::default() => 
                QueueNoiseOp::Modify(output.clone(), noise),
            QueueNoiseOp::Modify(input, noise) => QueueNoiseOp::Modify(input, noise),
            QueueNoiseOp::Combine(input1, input2, noise) if input1 == Handle::default() && input2 == Handle::default() =>
                QueueNoiseOp::Combine(output.clone(), output.clone(), noise),
            QueueNoiseOp::Combine(input1, input2, noise) => QueueNoiseOp::Combine(input1, input2, noise),
        }
    }
}

impl<T: ComputeNoise> From<T> for QueueNoiseOp {
    fn from(noise: T) -> Self {
        let erased = ErasedComputeNoise::from(noise);
        match T::NOISE_OP {
            NoiseOp::Generator => QueueNoiseOp::Generate(erased),
            NoiseOp::Modifier => QueueNoiseOp::Modify(Handle::default(), erased),
            NoiseOp::Combiner => QueueNoiseOp::Combine(Handle::default(), Handle::default(), erased),
        }
    }
}

pub trait IntoNoiseSequence {
    fn into_sequence(self, output: Handle<Image>) -> ComputeNoiseSequence;
}

impl IntoNoiseSequence for QueueNoiseOp {
    fn into_sequence(self, output: Handle<Image>) -> ComputeNoiseSequence {
        let instruction = match self.resolve_images(output.clone()) {
            QueueNoiseOp::Generate(noise) => ComputeNoiseInstruction {
                images: vec![output],
                noise,
                op: NoiseOp::Generator,
            },
            QueueNoiseOp::Modify(input, noise) => ComputeNoiseInstruction {
                images: vec![input, output],
                noise,
                op: NoiseOp::Modifier,
            },
            QueueNoiseOp::Combine(input1, input2, noise) => ComputeNoiseInstruction {
                images: vec![input1, input2, output],
                noise,
                op: NoiseOp::Combiner,
            },
        };
        ComputeNoiseSequence(vec![instruction])
    }
}

impl<T: ComputeNoise> IntoNoiseSequence for T {
    fn into_sequence(self, output: Handle<Image>) -> ComputeNoiseSequence {
        QueueNoiseOp::from(self).into_sequence(output)
    }
}

macro_rules! impl_into_noise_sequence_tuple {
    ($($idx:tt : $type:ident),+) => {
        impl<$($type: IntoNoiseSequence),+> IntoNoiseSequence for ($($type,)+) {
            fn into_sequence(self, image: Handle<Image>) -> ComputeNoiseSequence {
                let mut result = ComputeNoiseSequence(Vec::new());
                $(
                    let seq = self.$idx.into_sequence(image.clone());
                    result.0.extend(seq.0);
                )+
                result
            }
        }
    };
}

impl_into_noise_sequence_tuple! {0: A}
impl_into_noise_sequence_tuple! {0: A, 1: B}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K}
impl_into_noise_sequence_tuple! {0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I, 9: J, 10: K, 11: L}

// Main World
#[derive(Resource, Default)]
pub struct ComputeNoiseQueue {
    pub(crate) queue: Vec<ComputeNoiseSequence>,
}
impl ComputeNoiseQueue {
    pub fn queue<T: IntoNoiseSequence>(&mut self, output: Handle<Image>, operations: T) {
        self.queue.push(operations.into_sequence(output));
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
