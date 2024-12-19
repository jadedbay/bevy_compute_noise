use std::{any::TypeId, marker::PhantomData};

use bevy::{prelude::*, render::{render_resource::{binding_types::{texture_storage_2d, uniform_buffer_sized}, BindGroupLayout, BindGroupLayoutEntries, BindingType, BufferBindingType, CachedComputePipelineId, ComputePipelineDescriptor, DynamicBindGroupLayoutEntries, IntoBindGroupLayoutEntryBuilder, PipelineCache, ShaderDefVal, ShaderRef, ShaderStages, SpecializedComputePipeline, StorageTextureAccess, TextureDimension, TextureFormat, TextureViewDimension}, renderer::RenderDevice}, utils::HashMap};

use crate::noise::ComputeNoise;

pub struct ComputeNoiseTypePipeline<T: ComputeNoise> {
    pub noise_layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
    _phantom_data: PhantomData<T>,
}

impl<T: ComputeNoise> ComputeNoiseTypePipeline<T> {
    pub fn create_pipeline(world: &mut World) {
        let render_device = world.resource::<RenderDevice>().clone();

        let mut entries = DynamicBindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (T::bind_group_layout_entries()[0],)
        );
        for binding in T::bind_group_layout_entries().iter().skip(1) {
            entries = entries.extend_sequential(
                (*binding,)
            )
        }

        let noise_layout = render_device.create_bind_group_layout(
            Some("noise_layout".into()),
            &entries,
        );
        
        let image_layout = world.resource::<ComputeNoisePipelines>().get_image_layout(T::texture_dimension()).clone();

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

        let mut pipelines = world.resource_mut::<ComputeNoisePipelines>();
        pipelines.add_pipeline::<T>(Self {
            noise_layout,
            pipeline_id,
            _phantom_data: PhantomData,
        }.into());

        let mut entries = DynamicBindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (uniform_buffer_sized(false, None),)
        );
        for binding in T::bind_group_layout_entries().iter() {
            entries = entries.extend_sequential(
                (*binding,)
            )
        }
        let fbm_layout = render_device.create_bind_group_layout(
            Some("fbm_layout"),
            &entries,
        );

        let mut fbm_pipelines = world.resource_mut::<ComputeNoiseFbmPipeline>();
        fbm_pipelines.type_data.insert(TypeId::of::<T>(), (T::texture_dimension(), T::shader_def(), fbm_layout));
    }
}

pub struct ComputeNoisePipeline {
    pub noise_layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
}

impl<T: ComputeNoise> From<ComputeNoiseTypePipeline<T>> for ComputeNoisePipeline {
    fn from(compute_noise_pipeline: ComputeNoiseTypePipeline<T>) -> Self {
        ComputeNoisePipeline {
            noise_layout: compute_noise_pipeline.noise_layout,
            pipeline_id: compute_noise_pipeline.pipeline_id,
        }
    }
}

#[derive(Resource)]
pub struct ComputeNoiseFbmPipeline {
    pub image_2d_layout: BindGroupLayout,
    pub image_3d_layout: BindGroupLayout,
    pub type_data: HashMap<TypeId, (TextureDimension, ShaderDefVal, BindGroupLayout)>,
    shader: Handle<Shader>,
}

impl FromWorld for ComputeNoiseFbmPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        
        let image_2d_layout = render_device.create_bind_group_layout(
            "image_2d_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite),
            )
        );
        
        let image_3d_layout = render_device.create_bind_group_layout(
            "image_3d_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                BindingType::StorageTexture {
                    access: StorageTextureAccess::ReadWrite,
                    format: TextureFormat::Rgba8Unorm,
                    view_dimension: TextureViewDimension::D3,
                }.into_bind_group_layout_entry_builder(),
            )
        );
        
        let shader = world.resource::<AssetServer>().load("embedded://bevy_compute_noise/noise/shaders/fbm.wgsl");

        Self {
            image_2d_layout,
            image_3d_layout,
            type_data: HashMap::new(),
            shader,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FbmPipelineKey {
    pub noise_type_id: TypeId,
}
impl SpecializedComputePipeline for ComputeNoiseFbmPipeline {
    type Key = FbmPipelineKey;

    fn specialize(&self, key: Self::Key) -> ComputePipelineDescriptor {
        let type_data = self.type_data.get(&key.noise_type_id).unwrap();

        let image_layout = match type_data.0 {
            TextureDimension::D3 => self.image_3d_layout.clone(),
            _ => self.image_2d_layout.clone(),
        };

        let shader_defs = vec![type_data.1.clone()]; 

        ComputePipelineDescriptor {
            label: Some("fbm_pipeline".into()),
            layout: vec![image_layout.clone(), type_data.2.clone()],
            push_constant_ranges: Vec::new(),
            shader: self.shader.clone(),
            shader_defs,
            entry_point: "main".into(),
            zero_initialize_workgroup_memory: false,
        }        
    }
}

#[derive(Resource)]
pub struct ComputeNoisePipelines {
    pub image_2d_layout: BindGroupLayout,
    pub image_3d_layout: BindGroupLayout,
    pipelines: HashMap<TypeId, ComputeNoisePipeline>,
    _util_shader: Handle<Shader>,
}
impl ComputeNoisePipelines {
    pub fn get_pipeline(&self, type_id: TypeId) -> Option<&ComputeNoisePipeline> {
        self.pipelines.get(&type_id)
    }

    pub fn add_pipeline<T: ComputeNoise>(&mut self, pipeline: ComputeNoisePipeline) {
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
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite),
            )
        );
        let image_3d_layout = render_device.create_bind_group_layout(
            "image_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                BindingType::StorageTexture {
                    access: StorageTextureAccess::ReadWrite,
                    format: TextureFormat::Rgba8Unorm,
                    view_dimension: TextureViewDimension::D3,
                }.into_bind_group_layout_entry_builder(),
            )
        );

        Self {
            image_2d_layout,
            image_3d_layout,
            pipelines: HashMap::new(),
            _util_shader: world.resource::<AssetServer>().load("embedded://bevy_compute_noise/noise/shaders/util.wgsl"),
        }
    }
}

