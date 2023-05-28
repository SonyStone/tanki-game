use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::{path, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::components::*;
use crate::player::components::*;

#[derive(Bundle)]
struct TankBodyBundle {
    #[bundle]
    shape: ShapeBundle,
    fill: Fill,
    stroke: Stroke,
}

impl TankBodyBundle {
    fn new() -> Self {
        let body = shapes::Polygon {
            points: vec![
                Vec2::new(-20.0, -15.0),
                Vec2::new(-20.0, 15.0),
                Vec2::new(20.0, 15.0),
                Vec2::new(20.0, -15.0),
            ],
            closed: true,
        };

        let mut path_builder = path::PathBuilder::new();
        path_builder.move_to(Vec2::new(-20.0, -15.0));
        path_builder.line_to(Vec2::new(20.0, 0.0));
        path_builder.line_to(Vec2::new(-20.0, 15.0));
        let path = path_builder.build();

        let mut stroke = Stroke::new(Color::hex("191919").unwrap(), 2.0);
        stroke.options.line_join = LineJoin::Round;

        let mut fill = Fill::color(Color::hex("bf3030").unwrap());
        fill.options.fill_rule = FillRule::NonZero;

        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::new().add(&body).add(&path).build(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            fill,
            stroke,
        }
    }
}

impl Default for TankBodyBundle {
    fn default() -> Self {
        Self::new()
    }
}

pub fn setup_system(mut commands: Commands) {
    let turret = shapes::Circle {
        radius: 10.0,
        ..default()
    };

    let gun = shapes::Polygon {
        points: vec![
            Vec2::new(0.0, -5.0),
            Vec2::new(0.0, 5.0),
            Vec2::new(30.0, 5.0),
            Vec2::new(30.0, -5.0),
        ],
        closed: true,
    };

    let body = commands
        .spawn((
            Name::new("Player Body"),
            TankBodyBundle::new(),
            ExampleShape,
            Player {
                speed: 250.,
                pull_distance: 10.,
                ..default()
            },
            RigidBody::Dynamic,
            Damping {
                linear_damping: 50.0,
                angular_damping: 25.,
            },
            Velocity::default(),
            Collider::cuboid(20., 15.),
            Restitution::coefficient(0.7),
            ExternalImpulse::default(),
            KinematicCharacterController::default(),
        ))
        .id();

    let gun = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::new().add(&gun).build(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.1),
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::hex("bf3030").unwrap()),
            Stroke::new(Color::hex("191919").unwrap(), 2.0),
            ExampleShape,
            Name::new("Player Gun"),
            LookAt,
        ))
        .id();

    let turret = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::new().add(&turret).build(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.2),
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::hex("bf3030").unwrap()),
            Stroke::new(Color::hex("191919").unwrap(), 2.0),
            ExampleShape,
            Name::new("Player Turret"),
        ))
        .id();

    commands.entity(body).add_child(gun);
    commands.entity(gun).add_child(turret);

    let joint_controller = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(10.0, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    commands.spawn((
        Name::new("Player Controller"),
        ImpulseJoint::new(body, joint_controller),
        RigidBody::Dynamic,
        Collider::ball(5.),
        GlobalTransform::default(),
        Transform {
            translation: Vec3::new(10.0, 0.0, 0.0),
            ..default()
        },
        Velocity::default(),
        PlayerPull { speed: 15000. },
        Damping {
            linear_damping: 1.0,
            angular_damping: 100.,
        },
        ColliderMassProperties::Mass(0.5),
        CollisionGroups::new(Group::NONE, Group::NONE),
    ));
}

const NUMBER_OF_ENEMIES: i32 = 300;

pub fn spawn_enemies(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    for window in window_query.iter() {
        let enemy = shapes::Circle {
            radius: 10.0,
            ..default()
        };

        for _ in 0..NUMBER_OF_ENEMIES {
            let transform = Vec3::new(
                (random::<f32>() * window.width()) - window.width() / 2.0,
                (random::<f32>() * window.height()) - window.height() / 2.0,
                random::<f32>(),
            );

            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::new().add(&enemy).build(),
                    transform: Transform::from_translation(transform),
                    ..default()
                },
                Fill::color(Color::hex("bf3030").unwrap()),
                Stroke::new(Color::hex("191919").unwrap(), 2.0),
                ExampleShape,
                Name::new("Enemy"),
                RigidBody::Dynamic,
                Collider::ball(10.0),
                Restitution::coefficient(0.7),
                Damping {
                    linear_damping: 50.,
                    angular_damping: 10.0,
                },
            ));
        }
    }
}
