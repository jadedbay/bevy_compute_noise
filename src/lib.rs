use std::marker::PhantomData;

use bevy::{app::MainScheduleOrder, prelude::*, render::{extract_component::UniformComponentPlugin, extract_instances::ExtractInstancesPlugin, extract_resource::{extract_resource, ExtractResource, ExtractResourcePlugin}, render_asset::RenderAssets, render_graph::{self, RenderGraph, RenderLabel}, render_resource::{binding_types::{texture_2d, texture_3d, texture_storage_2d, uniform_buffer}, encase::internal::Error, BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BufferBinding, BufferInitDescriptor, BufferUsages, CachedComputePipelineId, CachedPipelineState, CommandEncoder, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, ShaderStages, ShaderType, StorageTextureAccess, TextureFormat, TextureSampleType}, renderer::{RenderDevice, RenderQueue}, MainWorld, Render, RenderApp, RenderSet}};
use bytemuck::{Pod, Zeroable};
use compute_noise::{ComputeNoise, Worley2D};

pub mod compute_noise;

pub struct ComputeNoisePlugin;

impl Plugin for ComputeNoisePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ComputeNoiseQueue::<Worley2D>::default())
            .add_plugins(ExtractResourcePlugin::<ComputeNoiseQueue<Worley2D>>::default())
            .add_systems(PreUpdate, clear_compute_noise_queue::<Worley2D>);

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .add_systems(Render, prepare_bind_groups.in_set(RenderSet::PrepareBindGroups));
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);    
        render_app
            .init_resource::<WorleyNoisePipeline>();
    }
}

fn clear_compute_noise_queue<T: ComputeNoise>(
    mut compute_noise_queue: ResMut<ComputeNoiseQueue<T>>,
) {
    compute_noise_queue.queue.clear();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct ComputeNodeLabel;

#[derive(Resource)]
pub struct WorleyNoisePipeline {
    pub layout: BindGroupLayout,
    pub pipeline_id: CachedComputePipelineId,
}

impl FromWorld for WorleyNoisePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "worley_noise_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
                    uniform_buffer::<Worley2D>(false),
                )
            )
        );

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/worley.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("worley_noise_compute".into()),
                layout: vec![layout.clone()],
                push_constant_ranges: Vec::new(),
                shader,
                shader_defs: vec![],
                entry_point: "worley".into(),
            });
            
        Self {
            layout,
            pipeline_id
        }
    }
}

#[derive(Resource, Clone, ExtractResource, Default)]
pub struct ComputeNoiseQueue<T: ComputeNoise> {
    queue: Vec<(Handle<Image>, T)>
}

impl<T: ComputeNoise> ComputeNoiseQueue<T> {
    pub fn add(&mut self, image: Handle<Image>, settings: T) -> Handle<Image> {
        self.queue.push((image.clone(), settings));

        image
    }
}

#[derive(Default, PartialEq)]
enum ComputeNoiseNodeState {
    #[default]
    Loading,
    Ready,
}

#[derive(Default)]
struct ComputeNoiseNode {
    state: ComputeNoiseNodeState,
}

impl render_graph::Node for ComputeNoiseNode {
    fn update(&mut self, world: &mut World) {
        if self.state == ComputeNoiseNodeState::Loading {
            let pipeline_cache = world.resource::<PipelineCache>();
            let worley_pipeline = world.resource::<WorleyNoisePipeline>();
            if let CachedPipelineState::Ok(_) = pipeline_cache.get_compute_pipeline_state(worley_pipeline.pipeline_id) {
                self.state = ComputeNoiseNodeState::Ready;
            } else {
                dbg!("loading");
            }
        }
    }

    fn run<'w>(
            &self,
            _graph: &mut render_graph::RenderGraphContext,
            render_context: &mut bevy::render::renderer::RenderContext<'w>,
            world: &'w World,
        ) -> Result<(), render_graph::NodeRunError> {
            if self.state == ComputeNoiseNodeState::Ready {
                let pipeline_cache = world.resource::<PipelineCache>();
                let worley_pipeline = world.resource::<WorleyNoisePipeline>();
                if let Some(pipeline) = pipeline_cache.get_compute_pipeline(worley_pipeline.pipeline_id) {
                    let mut pass = render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor::default());
    
                    pass.set_pipeline(pipeline);
                    if let Some(compute_noise_bind_groups) = world.get_resource::<ComputeNoiseBindGroups>() {
                        for (bind_group, workgroup_count) in compute_noise_bind_groups.worley.iter() {
                            pass.set_bind_group(0, &bind_group, &[]);
                            pass.dispatch_workgroups(workgroup_count.x as u32, workgroup_count.y as u32, 1);
            
                            dbg!("DISPATCHED");
                        }
                    }
                }
            }

            Ok(())
    }
}

#[derive(Default, Resource)]
struct ComputeNoiseRenderQueue<T: ComputeNoise> {
    queue: Vec<(BindGroup, Vec2)>,
    _phantom_data: PhantomData<T>,
}

#[derive(Resource)]
struct ComputeNoiseEncoder(Option<CommandEncoder>);

impl FromWorld for ComputeNoiseEncoder {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        ComputeNoiseEncoder(Some(render_device.create_command_encoder(&CommandEncoderDescriptor { 
            label: Some("compute_noise_encoder") 
        })))
    }
}

fn run_compute_noise_renderer<T: ComputeNoise>(
    mut compute_noise_renderer: ResMut<ComputeNoiseEncoder>,
    mut compute_noise_queue: ResMut<ComputeNoiseRenderQueue<T>>,
    pipeline_cache: Res<PipelineCache>,
    worley_pipeline: Res<WorleyNoisePipeline>,
    render_queue: Res<RenderQueue>,
    render_device: Res<RenderDevice>,
) {
    let mut dispatched = false;

    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(worley_pipeline.pipeline_id) {
        {
            let Some(encoder) = &mut compute_noise_renderer.0 else { return error!("Encoder is None") };
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            pass.set_pipeline(pipeline);
            
            for (bind_group, workgroup_count) in compute_noise_queue.queue.iter() {
                pass.set_bind_group(0, &bind_group, &[]);
                pass.dispatch_workgroups(workgroup_count.x as u32, workgroup_count.y as u32, 1);
                
                dispatched = true;
                dbg!("DISPATCHED");
            }
        }

        let encoder = compute_noise_renderer.0.take().unwrap();
        render_queue.submit(Some(encoder.finish()));

        compute_noise_renderer.0 = Some(render_device.create_command_encoder(&CommandEncoderDescriptor { 
            label: Some("compute noise encoder") 
        }));

        if dispatched { compute_noise_queue.queue.clear() };
    }
}

#[derive(Resource)]
struct ComputeNoiseBindGroups {
    pub worley: Vec<(BindGroup, Vec2)>,
}

fn prepare_bind_groups(
    mut commands: Commands,
    pipeline: Res<WorleyNoisePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    compute_noise: Res<ComputeNoiseQueue<Worley2D>>,
    render_device: Res<RenderDevice>,
    compute_noise_bind_groups: Option<ResMut<ComputeNoiseBindGroups>>,
    render_graph: Res<RenderGraph>,
) {
    let mut bind_groups = Vec::new();
    for (image_handle, settings) in compute_noise.queue.iter() {
        if let Some(image) = gpu_images.get(image_handle.clone()) {

            let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[settings.clone()]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
            });
            
            let bind_group = render_device.create_bind_group(
                Some("worley_noise_bind_group_layout".into()),
                &pipeline.layout,
                &BindGroupEntries::sequential((
                    &image.texture_view,
                    BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: None,
                    }
                )),
            );

            bind_groups.push((bind_group, image.size / 8.0));
        }
    }

    if render_graph.get_node::<ComputeNoiseNode>(ComputeNodeLabel).unwrap().state == ComputeNoiseNodeState::Loading {
        if let Some(mut compute_noise_bind_groups) = compute_noise_bind_groups {
            dbg!("hi");
            compute_noise_bind_groups.worley.extend(bind_groups.iter().cloned());
        } else {
            commands.insert_resource(ComputeNoiseBindGroups {
                worley: bind_groups,
            });
        }
    } else {
        commands.insert_resource(ComputeNoiseBindGroups {
            worley: bind_groups,
        });
    }
}