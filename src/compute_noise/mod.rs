use bevy::render::{render_resource::{BindGroup, BindGroupEntries, BindGroupLayout, ShaderRef}, renderer::RenderDevice};

pub mod worley_2d;
pub use worley_2d::Worley2D;

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone {
    type Gpu: GpuComputeNoise;
    
    fn gpu_data(&self, width: u32, height: u32) -> Self::Gpu;
    fn shader() -> ShaderRef;
    fn bind_group(&self, render_device: &RenderDevice, layout: &BindGroupLayout) -> BindGroup;
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout;
}

pub trait GpuComputeNoise {}
