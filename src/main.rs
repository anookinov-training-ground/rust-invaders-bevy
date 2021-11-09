#![allow(unused)]

use bevy::{prelude::*, window};

const PLAYER_SPRITE: &str = "player_a_01.png";

// Entity, Component, System, Resource

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(0, 0));

    // spawn a sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        ..Default::default()
    });
}
