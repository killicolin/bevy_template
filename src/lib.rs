use bevy::{
    prelude::{default, App, PluginGroup},
    window::{PresentMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};

pub fn run(width: f32, height: f32) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "GAME NAME".to_string(),
                width,
                height,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .run();
}
