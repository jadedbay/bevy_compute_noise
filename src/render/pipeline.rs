use std::{any::TypeId, marker::PhantomData};

use bevy::{prelude::*, render::{render_resource::{binding_types::texture_storage_2d, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBindingType, CachedComputePipelineId, ComputePipelineDescriptor, ComputePipelineId, IntoBindGroupLayoutEntryBuilder, PipelineCache, ShaderRef, ShaderStages, StorageTextureAccess, TextureDimension, TextureFormat, TextureViewDimension}, renderer::RenderDevice}, utils::HashMap};

use crate::noise::{ComputeNoise, Perlin2d, Worley2d, Worley3d};

#[derive(Resource, Clone)]
pub struct ComputeNoisePipeline<T: ComputeNoise> {
    pub noise_layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
    _phantom_data: PhantomData<T>,
}

impl<T: ComputeNoise> FromWorld for ComputeNoisePipeline<T> {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let texture_storage = match T::texture_dimension() {
            TextureDimension::D3 => BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D3,
            }.into_bind_group_layout_entry_builder(),
            _ => texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite),
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
        }.unwrap();

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_compute_pipeline(ComputePipelineDescriptor {
                label: None,
                layout: vec![image_layout.clone(), noise_layout.clone()],
                push_constant_ranges: Vec::new(),
                shader,
                shader_defs: vec![],
                entry_point: "noise".into(),
                zero_initialize_workgroup_memory: false,
            });

        Self {
            noise_layout,
            pipeline_id,
            _phantom_data: PhantomData,
        }
    }
}

pub struct CNPipeline {
    pub noise_layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
}

impl<T: ComputeNoise> From<ComputeNoisePipeline<T>> for CNPipeline {
    fn from(compute_noise_pipeline: ComputeNoisePipeline<T>) -> Self {
        CNPipeline {
            noise_layout: compute_noise_pipeline.noise_layout,
            pipeline_id: compute_noise_pipeline.pipeline_id,
        }
    }
}



#[derive(Resource)]
pub struct ComputeNoisePipelines {
    pub image_2d_layout: BindGroupLayout,
    pub image_3d_layout: BindGroupLayout,
    pipelines: HashMap<TypeId, CNPipeline>,
}
impl ComputeNoisePipelines {
    pub fn get_pipeline(&self, type_id: TypeId) -> Option<&CNPipeline> {
        self.pipelines.get(&type_id)
    }

    pub fn add_pipeline<T: ComputeNoise>(&mut self, pipeline: CNPipeline) {
        self.pipelines.insert(TypeId::of::<T>(), pipeline);
    }

    pub fn get_image_layout(&self, dimension: TextureDimension) -> &BindGroupLayout {
        match dimension {
            TextureDimension::D3 => &self.image_3d_layout,
            _ => &self.image_2d_layout,
        }
    }
}

impl FromWorld for ComputeNoisePipelines {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let image_2d_layout = render_device.create_bind_group_layout(
            "image_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite),
                    BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    }
                )
            )
        );
        let image_3d_layout = render_device.create_bind_group_layout(
            "image_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D3,
                    }.into_bind_group_layout_entry_builder(),
                    BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    }
                )
            )
        );

        let mut pipelines = HashMap::new();
        pipelines.insert(TypeId::of::<Perlin2d>(), ComputeNoisePipeline::<Perlin2d>::from_world(world).into());
        pipelines.insert(TypeId::of::<Worley2d>(), ComputeNoisePipeline::<Worley2d>::from_world(world).into());
        pipelines.insert(TypeId::of::<Worley3d>(), ComputeNoisePipeline::<Worley3d>::from_world(world).into());

        Self {
            image_2d_layout,
            image_3d_layout,
            pipelines,
        }
    }
}

pub(crate) fn initialize_pipelines(
    mut compute_noise_pipelines: ResMut<ComputeNoisePipelines>,
    perlin_2d: ComputeNoisePipeline<Perlin2d>,
    worley_2d: ComputeNoisePipeline<Worley2d>,
    worley_3d: ComputeNoisePipeline<Worley3d>,
) {
    compute_noise_pipelines.add_pipeline::<Perlin2d>(perlin_2d.into());
    compute_noise_pipelines.add_pipeline::<Worley2d>(worley_2d.into());
    compute_noise_pipelines.add_pipeline::<Worley3d>(worley_3d.into());
}