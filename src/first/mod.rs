pub mod components;
pub mod objects;
mod systems;

use bevy::prelude::{App, Plugin};
use bevy_prototype_lyon::prelude::ShapePlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_startup_system(setup_system)
            .add_startup_system(spawn_enemies);
    }
}
