use std::marker::PhantomData;

use bevy::{prelude::*, render::{render_resource::{binding_types::texture_storage_2d, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBindingType, CachedComputePipelineId, ComputePipelineDescriptor, IntoBindGroupLayoutEntryBuilder, PipelineCache, ShaderRef, ShaderStages, StorageTextureAccess, TextureDimension, TextureFormat, TextureViewDimension}, renderer::RenderDevice}};

use crate::compute_noise::ComputeNoise;

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

        let texture_storage = match T::texture_dimension() {
            TextureDimension::D3 => BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: TextureFormat::R8Unorm,
                view_dimension: TextureViewDimension::D3,
            }.into_bind_group_layout_entry_builder(),
            _ => texture_storage_2d(TextureFormat::R8Unorm, StorageTextureAccess::WriteOnly),
        };

        let image_layout = render_device.create_bind_group_layout(
            "image_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage,
                    BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    }
                )
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
