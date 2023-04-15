use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Component, Reflect, FromReflect, InspectorOptions,
)]
#[reflect(Component, PartialEq)]
pub struct Player {
    #[inspector(min = 0.0, max = 1000.0)]
    pub speed: f32,
    pub pull: Transform,
    #[inspector(min = 0.0, max = 100.0)]
    pub pull_distance: f32,
}

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Component, Reflect, FromReflect, InspectorOptions,
)]
#[reflect(Component, PartialEq)]
pub struct PlayerPull {
    #[inspector(min = 0.0, max = 1000.0)]
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct WorldCoords(pub Vec2);
