use bevy::{ecs::component::Component, prelude::*, reflect::{GetTypeRegistration, Reflect}, render::{render_graph::RenderLabel, render_resource::{BindGroup, BindGroupLayout, ShaderRef, TextureDimension}, renderer::RenderDevice}};

use crate::{image::ComputeNoiseSize, ComputeNoiseQueue};

pub mod worley_2d;
pub mod worley_3d;
pub mod perlin_2d;

pub use worley_2d::Worley2d;
pub use worley_3d::Worley3d;
pub use perlin_2d::Perlin2d;

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone + TypePath + FromReflect + GetTypeRegistration {
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

#[derive(Component, Reflect)]
pub struct ComputeNoiseComponent<T: ComputeNoise> {
    pub image: Handle<Image>,
    pub noise: T,
}

pub fn update_noise<T: ComputeNoise>(
    mut noise_queue: ResMut<ComputeNoiseQueue<T>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&ComputeNoiseComponent<T>, Changed<ComputeNoiseComponent<T>>>,
) {
    for noise in query.iter() {
        noise_queue.add_image(
            &mut images,
            noise.image.clone(),
            noise.noise.clone(),
        );
    }
}