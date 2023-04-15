use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::InspectorOptions;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
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
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(Msaa::Sample4)
        // OnEnter State Systems
        .add_startup_system(setup)
        .add_system(player_pull_movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PanCam::default()));

    let body = commands
        .spawn((
            Name::new("Player Body"),
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Damping {
                linear_damping: 50.0,
                angular_damping: 50.,
            },
            Velocity {
                linvel: Vec2::new(1., 2.),
                angvel: 0.4,
            },
            Collider::cuboid(20., 15.),
            Restitution::coefficient(0.7),
            ExternalImpulse::default(),
            KinematicCharacterController::default(),
            // ColliderMassProperties::Density(2.0),
        ))
        .id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(10.0, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(5.),
        GlobalTransform::default(),
        Transform {
            translation: Vec3::new(10.0, 0.0, 0.0),
            ..default()
        },
        Velocity::default(),
        Player {
            speed: 15000.,
            pull_distance: 10.,
            ..default()
        },
        Damping {
            linear_damping: 1.0,
            angular_damping: 100.,
        },
        ColliderMassProperties::Mass(0.5),
        CollisionGroups::new(Group::NONE, Group::NONE),
        ImpulseJoint::new(body, joint),
    ));
}

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

pub fn player_pull_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Velocity)>,
    time: Res<Time>,
) {
    for (player, mut velocity) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let key_directions = [
            (KeyCode::Left, Vec3::new(-1.0, 0.0, 0.0)),
            (KeyCode::A, Vec3::new(-1.0, 0.0, 0.0)),
            (KeyCode::Right, Vec3::new(1.0, 0.0, 0.0)),
            (KeyCode::D, Vec3::new(1.0, 0.0, 0.0)),
            (KeyCode::Up, Vec3::new(0.0, 1.0, 0.0)),
            (KeyCode::W, Vec3::new(0.0, 1.0, 0.0)),
            (KeyCode::Down, Vec3::new(0.0, -1.0, 0.0)),
            (KeyCode::S, Vec3::new(0.0, -1.0, 0.0)),
        ];

        for (key_code, dir) in key_directions.iter() {
            if keyboard_input.pressed(*key_code) {
                direction += *dir;
            }
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        let speed = player.speed;
        let delta_time = time.delta_seconds();

        velocity.linvel = direction.truncate() * speed * delta_time;
    }
}
