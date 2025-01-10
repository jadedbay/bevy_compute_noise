use std::any::TypeId;

use bevy::{prelude::*, render::{render_resource::{binding_types::{texture_storage_2d, uniform_buffer_sized}, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntryBuilder, BindingType, ComputePipelineDescriptor, IntoBindGroupLayoutEntryBuilder, ShaderDefVal, ShaderRef, ShaderStages, SpecializedComputePipeline, StorageTextureAccess, TextureDimension, TextureFormat, TextureViewDimension}, renderer::RenderDevice}, utils::HashMap};

use crate::noise::{ComputeNoiseType, Fbm};

pub fn load_compute_noise_shader<T: ComputeNoiseType>(world: &mut World) {
    let shader_2d = match T::shader_2d() {
        ShaderRef::Default => None,
        ShaderRef::Handle(handle) => Some(handle),
        ShaderRef::Path(path) => Some(world.resource::<AssetServer>().load(path)),
    }.unwrap();

    let shader_3d = match T::shader_3d() {
        ShaderRef::Default => None,
        ShaderRef::Handle(handle) => Some(handle),
        ShaderRef::Path(path) => Some(world.resource::<AssetServer>().load(path)),
    }.unwrap();

    let mut pipeline = world.resource_mut::<ComputeNoisePipeline>();

    pipeline.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<T>(),
            dimension: TextureDimension::D2,
        },
        shader_2d,
    );

    pipeline.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<T>(),
            dimension: TextureDimension::D3,
        },
        shader_3d,
    );
}

pub fn load_fbm_shaders<T: ComputeNoiseType>(world: &mut World) {
    let shader = world.resource::<AssetServer>().load("embedded://bevy_compute_noise/noise/shaders/fbm.wgsl");
    let mut pipeline = world.resource_mut::<ComputeNoisePipeline>();

    pipeline.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D2,
        },
        shader.clone(),
    );

    pipeline.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D3,
        },
        shader,
    );

    pipeline.shader_defs.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D2,
        },
        vec![T::shader_def(), "2D".into()]
    );
    pipeline.shader_defs.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D3,
        },
        vec![T::shader_def(), "3D".into()]
    );
}

#[derive(Resource)]
pub struct ComputeNoisePipeline {
    pub layout_2d: BindGroupLayout,
    pub layout_3d: BindGroupLayout,
    shaders: HashMap<ComputeNoisePipelineKey, Handle<Shader>>,
    shader_defs: HashMap<ComputeNoisePipelineKey, Vec<ShaderDefVal>>,
    _util_shader: Handle<Shader>,
}

impl FromWorld for ComputeNoisePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout_2d = render_device.create_bind_group_layout(
            "noise_2d_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    noise_texture_2d(),
                    uniform_buffer_sized(false, None),
                )
            )
        );
        let layout_3d = render_device.create_bind_group_layout(
            "noise_3d_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    noise_texture_3d(),
                    uniform_buffer_sized(false, None),
                )
            )
        );

        Self {
            layout_2d,
            layout_3d,
            shaders: HashMap::new(),
            shader_defs: HashMap::new(),
            _util_shader: world.resource::<AssetServer>().load("embedded://bevy_compute_noise/noise/shaders/util.wgsl"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputeNoisePipelineKey {
    pub type_id: TypeId,
    pub dimension: TextureDimension,
}

impl SpecializedComputePipeline for ComputeNoisePipeline {
    type Key = ComputeNoisePipelineKey;

    fn specialize(&self, key: Self::Key) -> ComputePipelineDescriptor {
        let layout = match key.dimension {
            TextureDimension::D2 => self.layout_2d.clone(),
            TextureDimension::D3 => self.layout_3d.clone(), 
            _ => unreachable!(),
        };

        ComputePipelineDescriptor {
            label: Some("compute_noise_pipeline".into()),
            layout: vec![layout],
            push_constant_ranges: Vec::new(),
            shader: self.shaders.get(&key).unwrap().clone(),
            shader_defs: self.shader_defs.get(&key).cloned().unwrap_or_default(),
            entry_point: "main".into(),
            zero_initialize_workgroup_memory: false,
        }
    }
}

pub fn noise_texture_2d() -> BindGroupLayoutEntryBuilder {
    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite)
}

pub fn noise_texture_3d() -> BindGroupLayoutEntryBuilder {
    BindingType::StorageTexture {
        access: StorageTextureAccess::ReadWrite,
        format: TextureFormat::Rgba8Unorm,
        view_dimension: TextureViewDimension::D3,
    }.into_bind_group_layout_entry_builder()
}