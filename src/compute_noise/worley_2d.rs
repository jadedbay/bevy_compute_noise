use bevy::{prelude::*, render::render_resource::ShaderRef};
use rand::Rng;
use super::{ComputeNoise, ComputeNoiseSettings};

#[derive(Default, Clone)]
#[repr(C)]
pub struct Worley2D {
    pub points: Vec<Vec2>,
}

impl Worley2D {
    fn generate_points(width: u32, height: u32, settings: Worley2DSettings) -> Vec<Vec2> {
        let mut rng = rand::thread_rng();

        let mut random_points = Vec::new();
        for _ in 0..settings.point_count {
            random_points.push(Vec2::new(rng.gen_range(0.0..width as f32), rng.gen_range(0.0..height as f32)));
        }

        random_points
    }
}

impl ComputeNoise for Worley2D {
    type Settings = Worley2DSettings;
    
    fn new(width: u32, height: u32, settings: Self::Settings) -> Self {
        Self {
            points: Worley2D::generate_points(width, height, settings),
        }
    }
    
    fn shader() -> ShaderRef {
        "shaders/worley_2d.wgsl".into()
    }
    
    fn as_slice(&self) -> &[u8] {
        bytemuck::cast_slice(self.points.as_slice())
    }
}
pub struct Worley2DSettings {
    pub point_count: u32,
}

impl Worley2DSettings {
    pub fn new(point_count: u32) -> Self {
        Self {
            point_count,
        }
    }
}

impl ComputeNoiseSettings for Worley2DSettings {}