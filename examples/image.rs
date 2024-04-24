use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let image = Image::new_fill(
        Extent3d {
            width: 4,
            height: 4,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0],
        TextureFormat::R8Unorm,
        RenderAssetUsages::MAIN_WORLD,
    );

    let handle = images.add(image);

    commands.spawn(handle);
}

fn update(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Handle<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        for mut handle in query.iter_mut() {
            let image = images.get_mut(handle.clone()).unwrap();

            for pixel in image.data.iter_mut() {
                *pixel = 255;
            }

            let new_image = image.clone();

            let new_handle = images.add(new_image);

            *handle = new_handle;

            dbg!("UPDATED");
        }
    }

    if input.just_pressed(KeyCode::KeyW) {
        for handle in query.iter() {
            commands.spawn(handle.clone());
            return;
        }
    }
}

