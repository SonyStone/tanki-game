use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::{ExternalImpulse, QueryFilter, RapierContext, Velocity};

use crate::{first::components::LookAt, MainCamera};

use super::components::*;

pub fn player_pull_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut PlayerPull, &mut Velocity)>,
    time: Res<Time>,
) {
    for (player, mut velocity) in player_query.iter_mut() {
        let mut direction = Vec2::ZERO;

        let key_directions = [
            (KeyCode::Left, Vec2::new(-1.0, 0.0)),
            (KeyCode::A, Vec2::new(-1.0, 0.0)),
            (KeyCode::Right, Vec2::new(1.0, 0.0)),
            (KeyCode::D, Vec2::new(1.0, 0.0)),
            (KeyCode::Up, Vec2::new(0.0, 1.0)),
            (KeyCode::W, Vec2::new(0.0, 1.0)),
            (KeyCode::Down, Vec2::new(0.0, -1.0)),
            (KeyCode::S, Vec2::new(0.0, -1.0)),
        ];

        key_directions.iter().for_each(|(key_code, dir)| {
            if keyboard_input.pressed(*key_code) {
                direction += *dir;
            }
        });

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        let speed = player.speed;
        let delta_time = time.delta_seconds();

        velocity.linvel = direction * speed * delta_time;
    }
}

use bevy::input::ButtonState;

fn look_at_z(
    local_transform: &mut Transform,
    global_transform: &GlobalTransform,
    cursor_position: &Vec2,
) {
    let global_transform = global_transform.compute_transform();

    let turret_position = global_transform.translation.truncate();
    let direction = *cursor_position - turret_position;
    let angle = direction.y.atan2(direction.x);

    let parent_rotation = global_transform.rotation * local_transform.rotation.inverse();
    local_transform.rotation = parent_rotation.inverse() * Quat::from_rotation_z(angle);
}

pub fn player_look_at(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut turret_query: Query<(&mut Transform, &GlobalTransform), With<LookAt>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_q.single();
    let cursor_position = get_cursor_position(window, camera, camera_transform);

    for (mut local_transform, global_transform) in turret_query.iter_mut() {
        if let Some(cursor_position) = cursor_position {
            look_at_z(&mut local_transform, global_transform, &cursor_position);
        }
    }
}

pub fn player_raycast(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands,
    rapier_context: ResMut<RapierContext>,
    mut lines: ResMut<DebugLines>,
    player_query: Query<(Entity, &Transform, &Player)>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_q.single();
    let cursor_position = get_cursor_position(window, camera, camera_transform);

    for ev in mousebtn_evr.iter() {
        if ev.state == ButtonState::Pressed {
            if let Some(cursor_position) = cursor_position {
                lines.line(
                    Vec3::from((cursor_position, 0.)),
                    Vec3::from((cursor_position + Vec2::new(10., 0.), 0.)),
                    0.0,
                );
                for (entity, transform, _) in player_query.iter() {
                    let ray_pos = Vec2::from((transform.translation.x, transform.translation.y));
                    let diff = cursor_position - ray_pos;
                    let ray_dir = diff.normalize();
                    let ray_pos = ray_pos + ray_dir * 30.;

                    let max_toi = diff.length();
                    let solid = true;
                    let filter = QueryFilter::default().exclude_rigid_body(entity);

                    lines.line(
                        ray_pos.extend(0.),
                        Vec3::from((ray_pos + ray_dir * max_toi, 0.)),
                        0.0,
                    );

                    if let Some((entity, toi)) =
                        rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                    {
                        // The first collider hit has the entity `entity` and it hit after
                        // the ray travelled a distance equal to `ray_dir * toi`.
                        let hit_point = ray_pos + ray_dir * toi;

                        commands.entity(entity).insert(ExternalImpulse::at_point(
                            ray_dir * 100.,
                            hit_point,
                            Vec2::ZERO,
                        ));
                    }
                }
            }
        };
    }
}

fn get_cursor_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}
