use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use big_brain::prelude::*;
use pancam::{PanCam, PanCamPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PanCamPlugin)
        .add_plugin(BigBrainPlugin)
        .add_startup_system(init_entities)
        .add_system(thirst_system)
        .add_systems(
            (drink_action_system, move_to_water_source_action_system).in_set(BigBrainSet::Actions),
        )
        .add_system(thirsty_scorer_system.in_set(BigBrainSet::Scorers))
        .run();
}

#[derive(Component, Debug)]
pub struct WaterSource {
    pub capacity: f32,
}

#[derive(Component, Debug)]
pub struct Thirst {
    pub per_second: f32,
    pub thirst: f32,
}

impl Thirst {
    pub fn new(thirst: f32, per_second: f32) -> Self {
        Self { thirst, per_second }
    }
}

pub fn thirst_system(time: Res<Time>, mut thirsts: Query<&mut Thirst>) {
    for mut thirst in &mut thirsts {
        thirst.thirst += thirst.per_second * time.delta_seconds();

        if thirst.thirst >= 100.0 {
            thirst.thirst = 100.0;
        }

        trace!("Thirst: {}", thirst.thirst);
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToWaterSource {
    speed: f32,
}

const MAX_DISTANCE: f32 = 0.1;

fn move_to_water_source_action_system(
    time: Res<Time>,
    mut waters: Query<(Entity, &Transform, &mut WaterSource, &mut Path)>,
    mut positions: Query<&mut Transform, Without<WaterSource>>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToWaterSource, &ActionSpan)>,
) {
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let mut actor_position = positions.get_mut(actor.0).expect("actor has no position");

                let closest_water_source = &waters.iter().min_by(|a, b| {
                    let da = (a.1.translation - actor_position.translation).length_squared();
                    let db = (b.1.translation - actor_position.translation).length_squared();

                    da.partial_cmp(&db).unwrap()
                });

                if let Some(closest_water_source) = closest_water_source {
                    let delta = closest_water_source.1.translation - actor_position.translation;

                    let distance = delta.length();

                    if distance > MAX_DISTANCE {
                        let step_size = time.delta_seconds() * move_to.speed;
                        let step = delta.normalize() * step_size.min(distance);

                        actor_position.translation += step;
                    } else {
                        *action_state = ActionState::Success;
                    }
                } else {
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink {
    per_second: f32,
}

fn drink_water(
    drink_amount: f32,
    thirst: &mut Thirst,
    water_source: &mut WaterSource,
    path: &mut Path,
    entity: Entity,
    commands: &mut Commands,
) {
    thirst.thirst -= drink_amount;
    water_source.capacity -= drink_amount;

    let circle = shapes::Circle {
        radius: water_source.capacity,
        ..default()
    };

    *path = GeometryBuilder::build_as(&circle);

    if thirst.thirst <= 0.0 {
        thirst.thirst = 0.0;
    }

    if water_source.capacity <= 0. {
        commands.entity(entity).despawn();
    }
}

fn drink_action_system(
    mut commands: Commands,
    time: Res<Time>,
    mut thirsts: Query<(&Transform, &mut Thirst), Without<WaterSource>>,
    mut waters: Query<(Entity, &Transform, &mut WaterSource, &mut Path)>,
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
) {
    for (Actor(actor), mut state, drink, span) in &mut query {
        let _guard = span.span().enter();

        let (actor_position, mut thirst) = thirsts.get_mut(*actor).expect("actor has no thirst");

        match *state {
            ActionState::Requested => {
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                let closest_water_source = &mut waters.iter_mut().min_by(|a, b| {
                    let da = (a.1.translation - actor_position.translation).length_squared();
                    let db = (b.1.translation - actor_position.translation).length_squared();

                    da.partial_cmp(&db).unwrap()
                });

                if let Some(closest_water_source) = closest_water_source {
                    let distance =
                        (closest_water_source.1.translation - actor_position.translation).length();

                    if distance < MAX_DISTANCE {
                        let drink_amout = drink.per_second * time.delta_seconds();
                        drink_water(
                            drink_amout,
                            &mut thirst,
                            &mut closest_water_source.2,
                            &mut closest_water_source.3,
                            closest_water_source.0,
                            &mut commands,
                        );
                        *state = if thirst.thirst == 0.0 {
                            ActionState::Success
                        } else {
                            ActionState::Failure
                        };
                    } else {
                        *state = ActionState::Failure;
                    }
                } else {
                    *state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;

pub fn thirsty_scorer_system(
    thirsts: Query<&Thirst>,
    mut query: Query<(&Actor, &mut Score), With<Thirsty>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(thirst) = thirsts.get(*actor) {
            score.set(thirst.thirst / 100.);
        }
    }
}

pub fn init_entities(mut commands: Commands) {
    let circle = shapes::Circle {
        radius: 5.0,
        ..default()
    };

    commands.spawn((Camera2dBundle::default(), PanCam::default()));

    let mut add_water_source = |x, y| {
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&circle),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            },
            Fill::color(Color::BLUE),
            Stroke::new(Color::BLACK, 0.1),
            WaterSource { capacity: 5.0 },
        ));
    };

    add_water_source(10., 10.);
    add_water_source(-10., 0.);
    add_water_source(-20., 0.);
    add_water_source(0., 20.);

    let thinker_circle = shapes::Circle {
        radius: 1.0,
        ..default()
    };

    let move_and_drink = Steps::build()
        .label("MoveAndDrink")
        .step(MoveToWaterSource { speed: 2.5 })
        .step(Drink { per_second: 10.0 });

    let thinker = Thinker::build()
        .label("ThirstyThinker")
        .picker(FirstToScore { threshold: 0.8 })
        .when(Thirsty, move_and_drink);

    commands.spawn((
        Thirst::new(75.0, 2.0),
        ShapeBundle {
            path: GeometryBuilder::build_as(&thinker_circle),
            transform: Transform::from_translation(Vec3::new(10., 0., 0.1)),
            ..default()
        },
        Fill::color(Color::RED),
        Stroke::new(Color::BLACK, 0.1),
        thinker,
    ));
}
