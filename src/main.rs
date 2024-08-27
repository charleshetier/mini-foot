use bevy::prelude::*;
use bevy_debug_grid::DebugGridPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod drop_ball_system;
mod move_attached_ball_to_owner_system;
mod move_camera_system;
mod move_free_ball_system;
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
                drop_ball_system::drop_ball
                    .run_if(move_attached_ball_to_owner_system::ball_is_owned),
                move_attached_ball_to_owner_system::attach_ball_to_owner
                    .run_if(move_attached_ball_to_owner_system::ball_is_owned),
                move_free_ball_system::move_free_ball
                    .run_if(move_attached_ball_to_owner_system::ball_is_not_owned),
                move_camera_system::move_camera,
            )
                .chain(),
        )
        .run();
}
