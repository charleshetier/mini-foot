use bevy::prelude::*;
use bevy_debug_grid::DebugGridPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod move_ball_system;
mod move_camera_system;
mod move_players_system;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DebugGridPlugin::with_floor_grid())
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            (
                move_players_system::move_players,
                move_ball_system::move_ball,
                move_camera_system::move_camera,
            )
                .chain(),
        )
        .run();
}
