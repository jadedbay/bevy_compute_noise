use bevy::render::render_resource::ShaderRef;

pub mod worley_2d;
pub use worley_2d::Worley2D;

pub trait ComputeNoise: Sync + Send + 'static + Default + Clone {
    type Settings: ComputeNoiseSettings;
    
    fn new(width: u32, height: u32, settings: Self::Settings) -> Self;
    fn shader() -> ShaderRef;
    fn as_slice(&self) -> &[u8];
}

pub trait ComputeNoiseSettings {}