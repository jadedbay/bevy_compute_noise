# bevy_compute_noise
[![crates.io](https://img.shields.io/crates/v/bevy_compute_noise.svg)](https://crates.io/crates/bevy_compute_noise)
[![Doc](https://docs.rs/bevy_compute_noise/badge.svg)](https://docs.rs/bevy_compute_noise)

A plugin for `bevy 0.14` for generating tilable 2D/3D noise textures using compute shaders.

Check out a demo of the plugin here: https://jadedbay.com/demo/bevy_compute_noise (Chrome only)

<img width="945" alt="bevy_compute_noise" src="https://github.com/jadedbay/bevy_compute_noise/assets/86005828/3d987e54-5846-47e0-ad97-262065b48596">

## Usage

Add the `bevy_compute_noise` dependency to `Cargo.toml`:

```toml
[dependencies]
bevy_compute_noise = "0.2.0"
```

### Add Noise Plugin
```rust
use bevy_compute_noise::prelude::*;

App::default()
    .add_plugins(DefaultPlugins)
    .add_plugins(ComputeNoisePlugin::<Perlin2D>::default()) // add new plugin for each type of noise needed
    .run();
```

### Queue Noise Generation
```rust
fn setup(
    mut images: ResMut<Assets<Image>>,
    mut perlin_2d_queue: ResMut<ComputeNoiseQueue<Perlin2D>>
) {
    // Create and queue noise image
    let noise_image: Handle<Image> = perlin_2d_queue.add(
        &mut images, 
        ComputeNoiseSize::D2(128, 128), // Use ComputeNoiseSize::D3 for 3D noise
        Perlin2d {
            seed: 0,
            frequency: 5,
            octaves: 4
        }
    );
}
```

Alternatively, you can add `ComputeNoiseComponent<T: ComputeNoise>` to an entity and it will be automatically queued whenever it has been updated:

```rust
fn setup(
    commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Manually create noise image, so it can be used elsewhere.
    let noise_image = ComputeNoiseImage::create_image(ComputeNoiseSize::D2(128, 128));
    
    commands.spawn(ComputeNoiseComponent::<Perlin2d> {
        image: noise_image.clone();
        noise: Perlin2d {
            seed: 0,
            frequency: 5,
            octaves: 4
        }
    });
}
```

## Noise Types
- Worley2D
- Worley3D
- Perlin2D

## TODO
- Add more noise types.
- Allow combination of noise.
- Allow writing into specific texture channels.

## Version Compatibility
| `bevy_compute_noise` | Bevy   |
| :--                  | :--    |
| `0.2`                | `0.14` |
| `0.1`                | `0.13` |

## Readback
If you need to readback the noise texture to the CPU, you can clone the readback branch and view the example in there. I'm not completely happy with the implementation and it's better to just generate the noise on the CPU using another crate anyway.
