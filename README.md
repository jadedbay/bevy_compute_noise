# bevy_compute_noise
[![crates.io](https://img.shields.io/crates/v/bevy_compute_noise.svg)](https://crates.io/crates/bevy_compute_noise)
[![Doc](https://docs.rs/bevy_compute_noise/badge.svg)](https://docs.rs/bevy_compute_noise)

A plugin for `bevy 0.15` for generating tilable 2D/3D noise textures using compute shaders.

Check out a demo of the plugin here: https://jadedbay.com/demo/bevy_compute_noise (This demo currently uses v0.1.0)

<img width="945" alt="bevy_compute_noise" src="https://github.com/jadedbay/bevy_compute_noise/assets/86005828/3d987e54-5846-47e0-ad97-262065b48596">

## Usage

Add the `bevy_compute_noise` dependency to `Cargo.toml`:

```toml
[dependencies]
bevy_compute_noise = "0.4.0"
```

### Add Noise Plugin
```rust
use bevy_compute_noise::prelude::*;

App::default()
    .add_plugins(DefaultPlugins)
    .add_plugins(ComputeNoisePlugin) // Add compute noise plugin
    .run();
```

### Write Noise to Image
```rust
fn setup(
    mut images: ResMut<Assets<Image>>,
    mut noise_queue: ResMut<ComputeNoiseQueue>
) {
    // Create image 
    let image = ComputeNoiseImage::create_image(ComputeNoiseSize::D2(512, 512));

    // Queue noise to be written to image
    noise_queue.generate(
        image
        Perlin {
            seed: 0,
            frequency: 5.0,
            flags: (PerlinFlags::default() | PerlinFlags::TILEABLE).bits()
        }
    );
}
```

## Noise Types
- Perlin
- Worley

FBM is available for all noise types, use `Fbm<T: ComputeNoiseGenerator>`.

## TODO
- Add more noise types.
- Allow combination of noise.
- Allow writing into specific texture channels.

## Version Compatibility
| `bevy_compute_noise` | Bevy   |
| :--                  | :--    |
| `0.3`, `0.4`         | `0.15` |
| `0.2`                | `0.14` |
| `0.1`                | `0.13` |
