use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::components::*;
use super::objects::*;
use crate::player::components::Player;

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let body = shapes::Polygon {
        points: vec![
            Vec2::new(-20.0, -15.0),
            Vec2::new(-20.0, 15.0),
            Vec2::new(20.0, 15.0),
            Vec2::new(20.0, -15.0),
        ],
        closed: true,
    };

    let body = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::new().add(&body).build(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::hex("bf3030").unwrap()),
            Stroke::new(Color::hex("191919").unwrap(), 2.0),
            ExampleShape,
            Player {
                speed: 250.,
                pull_distance: 10.,
                ..default()
            },
            Name::new("Player Body"),
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

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: (meshes.add(create_triangle())).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-120., 0., 0.)),
            ..default()
        },
        Name::new("Triangle"),
    ));
}

const NUMBER_OF_ENEMIES: i32 = 30;

pub fn spawn_enemies(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    for window in window_query.iter() {
        for _ in 0..NUMBER_OF_ENEMIES {
            let random = Vec3::new(
                (random::<f32>() * window.width()) - window.width() / 2.0,
                (random::<f32>() * window.height()) - window.height() / 2.0,
                random::<f32>(),
            );

            commands.spawn(create_element(random));
        }
    }
}
