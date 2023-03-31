use bevy::{
    prelude::{default, App, PluginGroup},
    window::{PresentMode, Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;

pub fn run(width: f32, height: f32) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "GAME_NAME".to_string(),
            resolution: WindowResolution::new(width, height),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }));
    if cfg!(debug_assertions) {
        app.add_plugin(EditorPlugin);
    }
    app.run();
}
