use bevy::{reflect::Reflect, render::{render_resource::{Buffer, BufferInitDescriptor, BufferUsages, TextureDimension}, renderer::RenderDevice}};
use bytemuck::{Pod, Zeroable};


use crate::noise::{ComputeNoise, ComputeNoiseType};

#[derive(Clone, Reflect, Default)] // TODO: manual default impl
pub struct Fbm<T: ComputeNoise> {
    pub noise: T,
    pub octaves: u32,
    pub lacunarity: f32,
    pub persistence: f32,
}

impl<T: ComputeNoiseType> ComputeNoise for Fbm<T> {
    fn texture_dimension() -> TextureDimension {
        T::texture_dimension()
    }
    fn buffers(&self, render_device: &RenderDevice) -> Vec<Buffer> {
        // let mut buffers = vec![    
        //     render_device.create_buffer_with_data(
        //         &BufferInitDescriptor {
        //             label: Some("perlin2d_buffer"),
        //             contents: &bytemuck::cast_slice(&[GpuFbm::from(self.clone())]),
        //             usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        //         }
        //     )
        // ];
        // buffers.extend(self.noise.buffers(render_device));
        // buffers

        // let mut buffer_data = Vec::new();
        
        // // Add FBM parameters (ensure proper alignment)
        // buffer_data.extend_from_slice(bytemuck::bytes_of(&self.octaves));
        // buffer_data.extend_from_slice(bytemuck::bytes_of(&self.lacunarity));
        // buffer_data.extend_from_slice(bytemuck::bytes_of(&self.persistence));
        
        // // Add padding to maintain alignment if needed
        // while buffer_data.len() % 16 != 0 {
        //     buffer_data.push(0);
        // }
        
        // // Add noise data
        // buffer_data.extend_from_slice(bytemuck::cast_slice(&[self.noise.clone()]));

        // vec![
        //     render_device.create_buffer_with_data(
        //         &BufferInitDescriptor {
        //             label: Some("combined_fbm_noise_buffer"),
        //             contents: &buffer_data,
        //             usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        //         }
        //     )
        // ]

        vec![
            render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: Some("combined_fbm_noise_buffer"),
                    contents: &[bytemuck::cast_slice(&[
                        self.octaves,
                        self.lacunarity.to_bits(),
                        self.persistence.to_bits(),
                        0u32,
                    ]),
                    bytemuck::cast_slice(&[self.noise.clone()])].concat(),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
                }
            )
        ]
    }
}

#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct GpuFbm {
    pub octaves: u32,
    pub lacunarity: f32,
    pub persistence: f32,
}

impl<T: ComputeNoiseType> From<Fbm<T>> for GpuFbm {
    fn from(value: Fbm<T>) -> Self {
        Self {
            octaves: value.octaves,
            lacunarity: value.lacunarity,
            persistence: value.persistence,
        }
    }
}