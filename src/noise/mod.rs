use bevy::{ecs::component::Component, prelude::*, reflect::Reflect, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupLayout, ShaderRef, TextureDimension}, renderer::RenderDevice}};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::{image::ComputeNoiseSize, readback::ComputeNoiseReadback, ComputeNoiseQueue};

pub mod worley_2d;
pub mod worley_3d;

pub use worley_2d::Worley2d;
pub use worley_3d::Worley3d;

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect {
    type Gpu: GpuComputeNoise;
    
    fn gpu_data(&self, size: ComputeNoiseSize) -> Self::Gpu;
    fn shader() -> ShaderRef;
    fn embed_asset(app: &mut App);
    fn render_label() -> impl RenderLabel;
    fn texture_dimension() -> TextureDimension;
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout;
}
pub trait GpuComputeNoise: Sync + Send + 'static + Default + Clone {
    fn bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup;
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct ComputeNoiseComponent<T: ComputeNoise> {
    pub image: Handle<Image>,
    pub noise: T,
}

pub fn update_noise<T: ComputeNoise>(
    mut noise_queue: ResMut<ComputeNoiseQueue<T>>,
    mut readback: Option<ResMut<ComputeNoiseReadback>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(&ComputeNoiseComponent<T>, Option<&ComputeNoiseAutoReadback>), Changed<ComputeNoiseComponent<T>>>,
) {
    for (noise, auto_readback) in query.iter() {
        noise_queue.add_image(
            &mut images,
            noise.image.clone(), 
            noise.noise.clone(), 
        );
        if auto_readback.is_some() {
            readback.as_mut()
                .expect("ComputeNoiseReadback resource does not exist, have you added the ComputeNoiseReadbackPlugin?")
                .queue(&mut images, noise.image.clone());
        }
    }
}

#[derive(Component)]
pub struct ComputeNoiseAutoReadback;

pub fn auto_readback_image<T: ComputeNoise>(
    mut images: ResMut<Assets<Image>>,
    readback: Res<ComputeNoiseReadback>,
    query: Query<&ComputeNoiseComponent<T>, With<ComputeNoiseAutoReadback>>,
) {
    for noise in query.iter() {
        if readback.senders.contains_key(&noise.image) {
            readback.receive(&mut images, noise.image.clone());
        }
    }
}