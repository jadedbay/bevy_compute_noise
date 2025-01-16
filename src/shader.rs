use std::borrow::Cow;

use bevy::{asset::Handle, render::render_resource::ShaderSource, utils::hashbrown::HashSet};

use crate::{noise_queue::{ComputeNoiseSequence, IntoNoiseSequence}, prelude::{Invert, Perlin}};

pub trait ComputeNoiseShader {
    fn import_path() -> &'static str;
    fn struct_name() -> Option<&'static str>;
    fn function_name() -> &'static str;
}

pub(crate) fn generate_noise_shader(sequence: &ComputeNoiseSequence) -> ShaderSource {
    let mut shader = String::new();
    add_imports(&mut shader, sequence);
    add_bindings(&mut shader, sequence);
    println!("{}", &shader);

    ShaderSource::Wgsl(shader.into())
}

fn add_imports(shader: &mut String, sequence: &ComputeNoiseSequence) {
    let mut imports = HashSet::new();

    for instruction in &sequence.0 {
        if imports.insert(instruction.noise.type_id) {
            if let Some(struct_name) = instruction.noise.struct_name {
                shader.push_str(&format!(
                    "#import {}::{{{}, {}}}\n",
                    instruction.noise.import_path,
                    struct_name,
                    instruction.noise.function_name
                ));
            } else {
                shader.push_str(&format!(
                    "#import {}::{}\n",
                    instruction.noise.import_path,
                    instruction.noise.function_name
                ));
            }
        }
    }
    shader.push('\n');
}

fn add_bindings(shader: &mut String, sequence: &ComputeNoiseSequence) {
    shader.push_str(
        "@group(0) @binding(0) var output: texture_storage_2d<rgba8unorm, read_write>;\n"
    );

    let mut binding_index = 1;
    for instruction in &sequence.0 {
        if instruction.images.len() > 1 {
            for _ in 1..instruction.images.len() {
                shader.push_str(&format!(
                    "@group(0) @binding({}) var input_texture{}: texture_storage_2d<rgba8unorm, read_write>;\n",
                    binding_index,
                    binding_index,
                ));
                binding_index += 1;
            }
        }

        if let Some(struct_name) = instruction.noise.struct_name {
            shader.push_str(&format!(
                "@group(0) @binding({}) var<uniform> {}{}: {};\n",
                binding_index,
                struct_name.to_lowercase(),
                binding_index,
                struct_name,
            ));
            binding_index += 1;
        }
    }
}

#[test]
fn test() {
    let sequence: ComputeNoiseSequence = (Perlin::default(), Perlin::default(), Invert).into_sequence(Handle::default());
    generate_noise_shader(&sequence);
}