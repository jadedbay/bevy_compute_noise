use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use rand::{rngs::StdRng, Rng};

use super::{ComputeNoise, GpuComputeNoise};

#[derive(Default, Clone, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Worley3d {
    cells: u32,
}

// impl Worley3d {
//     pub fn new(cells: u32) -> Self {
//         Self {
//             cells,
//         }
//     }

//     fn generate_points(&self, width: u32, height: u32, depth: u32) -> Vec<Vec3> {
//         let cell_size = (
//             width as f32 / self.cells as f32, 
//             height as f32 / self.cells as f32,
//             depth as f32 / self.cells as f32,
//         );

//         let mut rng = StdRng::from_seed()

//         let mut random_points = Vec::new();
//         for x in 0..self.cells {
//             for y in 0..self.cells {
//                 let x_range = (x as f32 * cell_size.0)..((x + 1) as f32 * cell_size.0);
//                 let y_range = (y as f32 * cell_size.1)..((y + 1) as f32 * cell_size.1);
//                 let z_range = (z as f32 * cell_size.2)..((y + 1) as f32 * cell_size.2);
//                 random_points.push(Vec3::new(rng.gen_range(x_range), rng.gen_range(y_range)));
//             }
//         }
//     }
// }
