pub mod first;
mod main_menu;
mod player;
mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use first::GamePlugin;
use main_menu::MainMenuPlugin;
use pancam::*;
use physics_2d_plugin::*;
use player::PlayerPlugin;
use systems::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera, PanCam::default()));
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::default())
           .add_plugin(LogDiagnosticsPlugin::default())
           .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(Msaa::Sample4)
        // OnEnter State Systems
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_camera)
        // .add_system(my_cursor_system)
        // .add_system(my_print_cursor_system)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(bevy::window::close_on_esc)
        .run();
}
