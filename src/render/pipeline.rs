use std::{any::TypeId, marker::PhantomData};

use bevy::{core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state, prelude::*, render::{render_resource::{binding_types::{storage_buffer_sized, texture_storage_2d, uniform_buffer_sized}, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntryBuilder, BindingType, CachedComputePipelineId, CachedRenderPipelineId, ColorTargetState, ColorWrites, ComputePipelineDescriptor, DynamicBindGroupLayoutEntries, FragmentState, IntoBindGroupLayoutEntryBuilder, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, ShaderDefVal, ShaderRef, ShaderStages, SpecializedComputePipeline, SpecializedComputePipelines, StorageTextureAccess, TextureDimension, TextureFormat, TextureViewDimension}, renderer::RenderDevice}, utils::HashMap};

use crate::noise::{ComputeNoise, ComputeNoiseType, Fbm};

// pub struct ComputeNoiseTypePipeline<T: ComputeNoiseType> {
//     pub noise_layout: BindGroupLayout,
//     pub pipeline_id: CachedComputePipelineId,
//     _phantom_data: PhantomData<T>,
// }

// impl<T: ComputeNoiseType> ComputeNoiseTypePipeline<T> {
//     pub fn create_pipeline(world: &mut World) {
//         let render_device = world.resource::<RenderDevice>().clone();

//         let mut entries = DynamicBindGroupLayoutEntries::sequential(
//             ShaderStages::COMPUTE,
//             (T::bind_group_layout_entries()[0],)
//         );
//         for binding in T::bind_group_layout_entries().iter().skip(1) {
//             entries = entries.extend_sequential(
//                 (*binding,)
//             )
//         }

//         let noise_layout = render_device.create_bind_group_layout(
//             Some("noise_layout".into()),
//             &entries,
//         );
        
//         let image_layout = world.resource::<ComputeNoisePipelines>().get_image_layout(T::texture_dimension()).clone();

//         let shader = match T::shader() {
            // ShaderRef::Default => None,
//             ShaderRef::Handle(handle) => Some(handle),
//             ShaderRef::Path(path) => Some(world.resource::<AssetServer>().load(path)),
//         }.unwrap();
        
//         let pipeline_id = world
//             .resource_mut::<PipelineCache>()
//             .queue_compute_pipeline(ComputePipelineDescriptor {
//                 label: None,
//                 layout: vec![image_layout.clone(), noise_layout.clone()],
//                 push_constant_ranges: Vec::new(),
//                 shader,
//                 shader_defs: vec![],
//                 entry_point: "main".into(),
//                 zero_initialize_workgroup_memory: false,
//             });

//         let mut pipelines = world.resource_mut::<ComputeNoisePipelines>();
//         pipelines.add_pipeline::<T>(Self {
//             noise_layout,
//             pipeline_id,
//             _phantom_data: PhantomData,
//         }.into());

//         let mut entries = DynamicBindGroupLayoutEntries::sequential(
//             ShaderStages::COMPUTE,
//             (uniform_buffer_sized(false, None),)
//         );
//         for binding in T::bind_group_layout_entries() {
//             entries = entries.extend_sequential(
//                 (binding,)
//             )
//         }
//         let fbm_layout = render_device.create_bind_group_layout(
//             Some("fbm_layout"),
//             &entries,
//         ); 

//         let mut fbm_pipelines = world.resource_mut::<ComputeNoiseFbmPipeline>();
//         fbm_pipelines.type_data.insert(TypeId::of::<T>(), (T::texture_dimension(), T::shader_def(), fbm_layout));
//     }

//     pub fn load_shader(world: &mut World) {
//         let shader = match T::shader() {
//             ShaderRef::Default => None,
//             ShaderRef::Handle(handle) => Some(handle),
//             ShaderRef::Path(path) => Some(world.resource::<AssetServer>().load(path)),
//         }.unwrap();

//         let mut pipelines = world.resource_mut::<ComputeNoisePipelines>();
//         pipelines.shaders.insert(TypeId::of::<T>(), shader);
//     }
// }

// TODO: Let user specify which shaders to load
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

    let mut pipelines = world.resource_mut::<ComputeNoisePipelines>();

    pipelines.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<T>(),
            dimension: TextureDimension::D2,
        },
        shader_2d,
    );

    pipelines.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<T>(),
            dimension: TextureDimension::D3,
        },
        shader_3d,
    );
}

pub fn load_fbm_shaders<T: ComputeNoiseType>(world: &mut World) {
    let shader = world.resource::<AssetServer>().load("embedded://bevy_compute_noise/noise/shaders/fbm.wgsl");
    let mut pipelines = world.resource_mut::<ComputeNoisePipelines>();

    pipelines.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D2,
        },
        shader.clone(),
    );

    pipelines.shaders.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D3,
        },
        shader,
    );

    pipelines.shader_defs.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D2,
        },
        vec![T::shader_def(), "2D".into()]
    );
    pipelines.shader_defs.insert(
        ComputeNoisePipelineKey {
            type_id: TypeId::of::<Fbm<T>>(),
            dimension: TextureDimension::D3,
        },
        vec![T::shader_def(), "3D".into()]
    );
}

// #[derive(Clone)]
// pub struct ComputeNoisePipeline {
//     pub noise_layout: BindGroupLayout,
//     pub pipeline_id: CachedComputePipelineId,
// }

// impl<T: ComputeNoiseType> From<ComputeNoiseTypePipeline<T>> for ComputeNoisePipeline {
//     fn from(compute_noise_pipeline: ComputeNoiseTypePipeline<T>) -> Self {
//         ComputeNoisePipeline {
//             noise_layout: compute_noise_pipeline.noise_layout,
//             pipeline_id: compute_noise_pipeline.pipeline_id,
//         }
//     }
// }

// #[derive(Resource)]
// pub struct ComputeNoiseFbmPipeline {
//     pub layout_2d: BindGroupLayout,
//     pub layout_3d: BindGroupLayout,
//     pub shader_defs: HashMap<TypeId, ShaderDefVal>,
//     shader: Handle<Shader>,
// }

// impl FromWorld for ComputeNoiseFbmPipeline {
//     fn from_world(world: &mut World) -> Self {
//         let render_device = world.resource::<RenderDevice>();
        
//         let layout_2d = render_device.create_bind_group_layout(
//             "noise_2d_layout",
//             &BindGroupLayoutEntries::sequential(
//                 ShaderStages::COMPUTE,
//                 (
//                     noise_texture_2d(),
//                     uniform_buffer_sized(false, None),
//                     uniform_buffer_sized(false, None),
//                 )
//             )
//         );
//         let layout_3d = render_device.create_bind_group_layout(
//             "noise_3d_layout",
//             &BindGroupLayoutEntries::sequential(
//                 ShaderStages::COMPUTE,
//                 (
//                     noise_texture_3d(),
//                     uniform_buffer_sized(false, None),
//                     uniform_buffer_sized(false, None),
//                 )
//             )
//         );
        
//         let shader = world.resource::<AssetServer>().load("embedded://bevy_compute_noise/noise/shaders/fbm.wgsl");

//         Self {
//             layout_2d,
//             layout_3d,
//             shader_defs: HashMap::new(),
//             shader,
//         }
//     }
// }

// impl SpecializedComputePipeline for ComputeNoiseFbmPipeline {
//     type Key = ComputeNoisePipelineKey;

//     fn specialize(&self, key: Self::Key) -> ComputePipelineDescriptor {
//         let shader_def = self.type_data.get(&key.type_id).unwrap();

//         let mut shader_defs = vec![type_data.0.clone()];

//         match key.dimension {
//             TextureDimension::D3 => {
//                 shader_defs.push("3D".into());
//             }
//             _ => {
//                 shader_defs.push("2D".into());
//             },
//         };

//         ComputePipelineDescriptor {
//             label: Some("fbm_pipeline".into()),
//             layout: vec![type_data.1.clone()],
//             push_constant_ranges: Vec::new(),
//             shader: self.shader.clone(),
//             shader_defs,
//             entry_point: "main".into(),
//             zero_initialize_workgroup_memory: false,
//         }        
//     }
// }

#[derive(Resource)]
pub struct ComputeNoisePipelines {
    pub layout_2d: BindGroupLayout,
    pub layout_3d: BindGroupLayout,
    shaders: HashMap<ComputeNoisePipelineKey, Handle<Shader>>,
    shader_defs: HashMap<ComputeNoisePipelineKey, Vec<ShaderDefVal>>,
    _util_shader: Handle<Shader>,
}

impl FromWorld for ComputeNoisePipelines {
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

impl SpecializedComputePipeline for ComputeNoisePipelines {
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