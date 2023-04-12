use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;

pub fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[10.0, 0.0, 0.0], [0.0, 10.0, 0.0], [10.0, 10.0, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

pub fn create_element(
    transform: Vec3,
) -> (
    ShapeBundle,
    Fill,
    Stroke,
    ExampleShape,
    Name,
    RigidBody,
    Collider,
    Restitution,
    Damping,
) {
    let enemy = shapes::Circle {
        radius: 10.0,
        ..default()
    };

    (
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
            linear_damping: 0.5,
            angular_damping: 10.0,
        },
    )
}
