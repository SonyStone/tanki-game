pub mod components;
mod systems;

use bevy::prelude::{App, Plugin};

use components::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            // .add_system(player_movement)
            .add_system(player_pull_movement)
            .add_system(my_cursor_system)
            .add_startup_system(setup_world_coords)
            .add_system(player_raycast)
            .add_system(player_lookat);
    }
}
