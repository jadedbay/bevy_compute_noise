use std::marker::PhantomData;

use bevy::{prelude::*, render::{render_resource::{binding_types::{storage_buffer_read_only, texture_storage_2d, uniform_buffer}, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingType, BufferBindingType, CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache, ShaderRef, ShaderStages, StorageTextureAccess, TextureFormat}, renderer::RenderDevice}};

use crate::compute_noise::{ComputeNoise, worley_2d::Worley2D};

#[derive(Resource)]
pub struct ComputeNoisePipeline<T: ComputeNoise> {
    pub layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
    _phantom_data: PhantomData<T>,
}

impl<T: ComputeNoise> FromWorld for ComputeNoisePipeline<T> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "worley_noise_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
                    BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                )
            )
        );

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
                layout: vec![layout.clone()],
                push_constant_ranges: Vec::new(),
                shader,
                shader_defs: vec![],
                entry_point: "noise".into(),
            });
            
        Self {
            layout,
            pipeline_id,
            _phantom_data: PhantomData,
        }
    }
}
