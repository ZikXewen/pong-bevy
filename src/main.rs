#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{thread_rng, Rng};

const TOP_WALL: f32 = 300.;
const BOTTOM_WALL: f32 = -300.;
const LEFT_WALL: f32 = -500.;
const RIGHT_WALL: f32 = 500.;
const WALL_THICKNESS: f32 = 10.;
const WALL_PADDING: f32 = 15.;
const PADDLE_SIZE: Vec3 = const_vec3!([15., 100., 0.]);
const PADDLE_SPEED: f32 = 450.;
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_INITIAL_SPEED: f32 = 400.;
const BALL_SPEED: f32 = 600.;
const FONT_SIZE: f32 = 40.0;
const TEXT_PADDING: Val = Val::Px(5.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_startup_system(setup_system)
        .add_system(movement_system)
        .add_system(ball_system)
        .add_system(scoreboard_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    //* Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    //* Walls
    commands
        .spawn()
        .insert(LeftGoal)
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([LEFT_WALL, 0., 0.]),
                scale: const_vec3!([WALL_THICKNESS, TOP_WALL - BOTTOM_WALL + WALL_THICKNESS, 0.]),
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(RightGoal)
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([RIGHT_WALL, 0., 0.]),
                scale: const_vec3!([WALL_THICKNESS, TOP_WALL - BOTTOM_WALL + WALL_THICKNESS, 0.]),
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([0., TOP_WALL, 0.]),
                scale: const_vec3!([RIGHT_WALL - LEFT_WALL + WALL_THICKNESS, WALL_THICKNESS, 0.]),
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([0., BOTTOM_WALL, 0.]),
                scale: const_vec3!([RIGHT_WALL - LEFT_WALL + WALL_THICKNESS, WALL_THICKNESS, 0.]),
                ..default()
            },
            ..default()
        });

    //* Paddles
    commands
        .spawn()
        .insert(LeftPaddle)
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([LEFT_WALL + WALL_PADDING, 0., 0.]),
                scale: PADDLE_SIZE,
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(RightPaddle)
        .insert(Collider)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([RIGHT_WALL - WALL_PADDING, 0., 0.]),
                scale: PADDLE_SIZE,
                ..default()
            },
            ..default()
        });

    //* Ball
    commands
        .spawn()
        .insert(Velocity(
            const_vec2!([
                thread_rng().gen_range(0.3..=1.0),
                thread_rng().gen_range(-1.0..=1.0)
            ])
            .normalize()
                * BALL_INITIAL_SPEED,
        ))
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: const_vec3!([0., 0., 0.]),
                scale: BALL_SIZE,
                ..default()
            },
            ..default()
        });

    //* Scoreboard
    commands
        .spawn()
        .insert(LeftScoreboard)
        .insert_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_owned(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Inter-Medium.ttf"),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        ..default()
                    },
                    TextSection {
                        value: "0".to_owned(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Inter-Regular.ttf"),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        ..default()
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: TEXT_PADDING,
                    left: TEXT_PADDING,
                    ..default()
                },
                ..default()
            },
            ..default()
        });
    commands
        .spawn()
        .insert(RightScoreboard)
        .insert_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_owned(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Inter-Medium.ttf"),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        ..default()
                    },
                    TextSection {
                        value: "0".to_owned(),
                        style: TextStyle {
                            font: asset_server.load("fonts/Inter-Regular.ttf"),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        ..default()
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: TEXT_PADDING,
                    right: TEXT_PADDING,
                    ..default()
                },
                ..default()
            },
            ..default()
        });
}

fn movement_system(
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
    mut left_paddle: Query<&mut Transform, With<LeftPaddle>>,
    mut right_paddle: Query<&mut Transform, (With<RightPaddle>, Without<LeftPaddle>)>,
) {
    let mut left_dir = 0.;
    let mut right_dir = 0.;
    if key.pressed(KeyCode::W) {
        left_dir += 1.
    }
    if key.pressed(KeyCode::S) {
        left_dir -= 1.
    }
    if key.pressed(KeyCode::Up) {
        right_dir += 1.
    }
    if key.pressed(KeyCode::Down) {
        right_dir -= 1.
    }
    let mut transform_left = left_paddle.single_mut();
    let mut transform_right = right_paddle.single_mut();
    let left_new_y = transform_left.translation.y + left_dir * time.delta_seconds() * PADDLE_SPEED;
    let right_new_y =
        transform_right.translation.y + right_dir * time.delta_seconds() * PADDLE_SPEED;
    transform_left.translation.y = left_new_y.clamp(
        BOTTOM_WALL + (PADDLE_SIZE.y * 0.5),
        TOP_WALL - (PADDLE_SIZE.y * 0.5),
    );
    transform_right.translation.y = right_new_y.clamp(
        BOTTOM_WALL + (PADDLE_SIZE.y * 0.5),
        TOP_WALL - (PADDLE_SIZE.y * 0.5),
    );
}

fn ball_system(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut ball: Query<(&mut Velocity, &mut Transform)>,
    collider: Query<
        (&Transform, Option<&LeftGoal>, Option<&RightGoal>),
        (With<Collider>, Without<Velocity>),
    >,
) {
    let (mut ball_velocity, mut ball_transform) = ball.single_mut();
    ball_transform.translation.x += ball_velocity.x * time.delta_seconds();
    ball_transform.translation.y += ball_velocity.y * time.delta_seconds();
    for (transform, left_goal, right_goal) in collider.iter() {
        let collision = collide(
            ball_transform.translation,
            BALL_SIZE.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            if left_goal.is_some() {
                score.right += 1;
                ball_transform.translation = const_vec3!([0., 0., 0.]);
                let new_velocity = const_vec2!([
                    thread_rng().gen_range(-1.0..=-0.3),
                    thread_rng().gen_range(-1.0..=1.0)
                ])
                .normalize()
                    * BALL_INITIAL_SPEED;
                ball_velocity.x = new_velocity.x;
                ball_velocity.y = new_velocity.y;
                return;
            }
            if right_goal.is_some() {
                score.left += 1;
                ball_transform.translation = const_vec3!([0., 0., 0.]);
                let new_velocity = const_vec2!([
                    thread_rng().gen_range(0.3..=1.0),
                    thread_rng().gen_range(-1.0..=1.0)
                ])
                .normalize()
                    * BALL_INITIAL_SPEED;
                ball_velocity.x = new_velocity.x;
                ball_velocity.y = new_velocity.y;
                return;
            }
            let mut reflect_x = false;
            let mut reflect_y = false;
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.,
                Collision::Right => reflect_x = ball_velocity.x < 0.,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.,
                Collision::Top => reflect_y = ball_velocity.y < 0.,
                _ => (),
            }
            if reflect_x {
                let mut new_velocity = const_vec2!([
                    thread_rng().gen_range(0.3..=1.0),
                    thread_rng().gen_range(-1.0..=1.0)
                ])
                .normalize()
                    * BALL_SPEED;
                if ball_velocity.x > 0. {
                    new_velocity.x = -new_velocity.x;
                }
                ball_velocity.x = new_velocity.x;
                ball_velocity.y = new_velocity.y;
            }
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn scoreboard_system(
    score: Res<Score>,
    mut left_scoreboard: Query<&mut Text, With<LeftScoreboard>>,
    mut right_scoreboard: Query<&mut Text, (With<RightScoreboard>, Without<LeftScoreboard>)>,
) {
    let mut left_scoreboard = left_scoreboard.single_mut();
    let mut right_scoreboard = right_scoreboard.single_mut();
    left_scoreboard.sections[1].value = score.left.to_string();
    right_scoreboard.sections[1].value = score.right.to_string();
}

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct LeftGoal;

#[derive(Component)]
struct RightGoal;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct LeftScoreboard;

#[derive(Component)]
struct RightScoreboard;

#[derive(Default)]
struct Score {
    left: u32,
    right: u32,
}
