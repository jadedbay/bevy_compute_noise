use super::ComputeNoise;

pub struct Fbm<T: ComputeNoise> {
    pub noise: T,
    pub octaves: usize,
    pub frequency: f32,
    pub lacunarity: f32,
    pub persistence: f32,
}