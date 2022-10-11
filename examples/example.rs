use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let s = 100.0 * Vec3::X;
    for (translation, image_path) in [(-s, "spriteA.png"), (s, "spriteB.png")] {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(100.0)),
                ..Default::default()
            },
            texture: asset_server.load(image_path),
            transform: Transform::from_translation(translation),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_image_config::ImageConfigPlugin)
        .add_startup_system(setup)
        .run();
}
