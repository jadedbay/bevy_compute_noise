
use bevy::{prelude::*, render::{extract_resource::{ExtractResource, ExtractResourcePlugin}, render_asset::RenderAssets, render_graph::{self, RenderGraph, RenderLabel}, render_resource::{binding_types::{texture_2d, texture_3d, texture_storage_2d}, BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingType, CachedComputePipelineId, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, PipelineCache, PipelineLayoutDescriptor, RawComputePipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, ShaderStages, StorageTextureAccess, TextureFormat, TextureSampleType, TextureViewDimension}, renderer::RenderDevice, Render, RenderApp, RenderSet}};

pub struct ComputeNoisePlugin;

impl Plugin for ComputeNoisePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ComputeNoise::default())
            .add_plugins(ExtractResourcePlugin::<ComputeNoise>::default());

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(Render, prepare_bind_groups.in_set(RenderSet::PrepareBindGroups));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(ComputeNodeLabel, ComputeNoiseNode::default());
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);    
        render_app.init_resource::<WorleyNoisePipeline>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct ComputeNodeLabel;

#[derive(Resource)]
pub struct WorleyNoisePipeline {
    pub layout: BindGroupLayout,
    pub pipeline: ComputePipeline,
}

impl FromWorld for WorleyNoisePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "worley_noise_bind_group_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
            )
        );
        
        let shader_source = include_str!("../assets/shaders/worley.wgsl");
        let shader = render_device.create_shader_module(ShaderModuleDescriptor {
            label: Some("worley_shader"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("worley_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let pipeline = render_device.create_compute_pipeline(&RawComputePipelineDescriptor {
            label: Some("worley_noise_compute".into()),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "worley".into(),
        });
            
        Self {
            layout,
            pipeline
        }
    }
}

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoise {
    worley: Vec<(Handle<Image>, WorleySettings)>
}

impl ComputeNoise {
    pub fn generate_worley(&mut self, image: Handle<Image>, settings: WorleySettings) -> Handle<Image> {
        self.worley.push((image.clone(), settings));

        image
    }
}

#[derive(Clone, Copy)]
pub struct WorleySettings {
    pub point_count: u32,
}

#[derive(Default)]
struct ComputeNoiseNode;

impl render_graph::Node for ComputeNoiseNode {
    fn run<'w>(
            &self,
            _graph: &mut render_graph::RenderGraphContext,
            render_context: &mut bevy::render::renderer::RenderContext<'w>,
            world: &'w World,
        ) -> Result<(), render_graph::NodeRunError> {
            if let Some(compute_noise_bind_group) = world.get_resource::<ComputeNoiseBindGroup>() {
                let pipeline_cache = world.resource::<PipelineCache>();
                let worley_pipeline = world.resource::<WorleyNoisePipeline>();
                // if let Some(pipeline) = pipeline_cache.get_compute_pipeline(worley_pipeline.pipeline_id) {
                //     let mut pass = render_context
                //         .command_encoder()
                //         .begin_compute_pass(&ComputePassDescriptor::default());
    
                //     pass.set_pipeline(pipeline);
                //     pass.set_bind_group(0, &compute_noise_bind_group.0, &[]);
                //     pass.dispatch_workgroups(1, 1, 1);

                //     // let mut compute_noise = world.get_resource_mut::<ComputeNoise>().unwrap();
                //     // compute_noise.worley.remove(0);
    
                //     dbg!("DISPATCHED");
                // }
            }

            Ok(())
    }
}

#[derive(Resource)]
struct ComputeNoiseBindGroup(BindGroup);

fn prepare_bind_groups(
    mut commands: Commands,
    pipeline: Res<WorleyNoisePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    mut compute_noise: ResMut<ComputeNoise>,
    render_device: Res<RenderDevice>,
) {
    if !compute_noise.worley.is_empty() {
        let image = gpu_images.get(compute_noise.worley[0].0.clone()).unwrap();
        let bind_group = render_device.create_bind_group(
            Some("worley_noise_bind_group_layout".into()),
            &pipeline.layout,
            &BindGroupEntries::single(&image.texture_view)
        );

        compute_noise.worley.remove(0);

        commands.insert_resource(ComputeNoiseBindGroup(bind_group));
    }
}