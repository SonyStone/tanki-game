use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, KinematicCharacterController};
use pancam::*;
use physics_2d_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(Msaa::Sample4)
        // OnEnter State Systems
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PanCam::default()));

    commands.spawn((
        Name::new("Player Body"),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.,
        },
        Velocity {
            linvel: Vec2::new(1., 2.),
            angvel: 0.4,
        },
        Collider::cuboid(20., 15.),
        Restitution::coefficient(0.7),
        ExternalImpulse::default(),
        KinematicCharacterController::default(),
    ));
}
