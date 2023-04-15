use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::new(0., 0.),
                ..default()
            })
            .add_startup_system(setup_physics);
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands.spawn((
        Name::new("Ground"),
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
    ));

    /* Create the bouncing ball. */
    commands.spawn((
        Name::new("Bouncing Ball"),
        RigidBody::Dynamic,
        Collider::ball(50.0),
        Restitution::coefficient(0.7),
        Damping {
            linear_damping: 50.0,
            angular_damping: 100.,
        },
        TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)),
    ));
}
