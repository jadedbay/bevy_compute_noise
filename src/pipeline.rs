use std::marker::PhantomData;

use bevy::{prelude::*, render::{render_resource::{binding_types::{storage_buffer_read_only, texture_storage_2d, uniform_buffer}, BindGroup, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingType, BufferBindingType, CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache, ShaderRef, ShaderStages, SpecializedComputePipeline, StorageTextureAccess, TextureFormat}, renderer::RenderDevice}};

use crate::compute_noise::{ComputeNoise, worley_2d::Worley2D};

#[derive(Resource)]
pub struct ComputeNoisePipeline<T: ComputeNoise> {
    pub image_layout: BindGroupLayout,
    pub noise_layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
    _phantom_data: PhantomData<T>,
}

impl<T: ComputeNoise> FromWorld for ComputeNoisePipeline<T> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let image_layout = render_device.create_bind_group_layout(
            "image_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                texture_storage_2d(TextureFormat::R8Unorm, StorageTextureAccess::WriteOnly),
            )
        );

        let noise_layout = T::bind_group_layout(render_device);

        let shader = match T::shader() {
                ShaderRef::Default => None,
                ShaderRef::Handle(handle) => Some(handle),
                ShaderRef::Path(path) => Some(world.resource::<AssetServer>().load(path)),
            }
            .unwrap();

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_compute_pipeline(ComputePipelineDescriptor {
                label: None,
                layout: vec![image_layout.clone(), noise_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader,
                shader_defs: vec![],
                entry_point: "noise".into(),
            });
            
        Self {
            image_layout,
            noise_layout,
            pipeline_id,
            _phantom_data: PhantomData,
        }
    }
}
