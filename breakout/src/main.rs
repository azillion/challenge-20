use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;
const BALL_RADIUS: f32 = 7.0;
const BALL_VELOCITY: f32 = 200.0;

pub struct BreakoutPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BreakoutPlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .run();
}

#[derive(Resource)]
struct Arena {
    width: f32,
    height: f32,
    wall_thickness: f32,
}

#[derive(Resource)]
struct Score(usize);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct LivesText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Collision;

fn startup(windows: Query<&Window>, mut arena: ResMut<Arena>) {
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();
    let wall_thickness = 4.;
    let arena_width = (window_width - wall_thickness * 2.) * 0.6;
    let arena_height = (window_height - wall_thickness * 2.) * 0.9;

    arena.width = arena_width;
    arena.height = arena_height;
    arena.wall_thickness = wall_thickness;
}

fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    arena: Res<Arena>,
) {
    // Top Wall
    commands.spawn((
        Collision,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.width, arena.wall_thickness))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., arena.height / 2. + arena.wall_thickness / 2., 0.),
            ..Default::default()
        },
    ));

    // Bottom Wall
    commands.spawn((
        Collision,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.width, arena.wall_thickness))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., -arena.height / 2. - arena.wall_thickness / 2., 0.),
            ..Default::default()
        },
    ));

    // Left Wall
    commands.spawn((
        Collision,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.wall_thickness, arena.height))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-arena.width / 2. - arena.wall_thickness / 2., 0., 0.),
            ..Default::default()
        },
    ));

    // Right Wall
    commands.spawn((
        Collision,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.wall_thickness, arena.height))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(arena.width / 2. + arena.wall_thickness / 2., 0., 0.),
            ..Default::default()
        },
    ));
}

fn setup_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    arena: Res<Arena>,
) {
    let paddle_padding = 10.;
    commands.spawn((
        Paddle,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                -arena.width / 2. + PADDLE_WIDTH / 2. + paddle_padding,
                0.,
                0.,
            ),
            ..Default::default()
        },
    ));
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let velocity: f32;
    if rand::random() {
        velocity = BALL_VELOCITY;
    } else {
        velocity = -BALL_VELOCITY;
    }

    commands.spawn((
        Ball,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BALL_RADIUS, BALL_RADIUS))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Velocity { x: velocity, y: 0. },
    ));
}

fn setup_score(mut commands: Commands, arena: Res<Arena>) {
    commands.spawn((
        ScoreText,
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: 50.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(arena.height / 2. - 50.),
                left: Val::Px(arena.width / 2. - 50.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn ball_move_system(
    time: Res<Time>,
    mut set: ParamSet<(
        Query<(&mut Transform, &mut Velocity), With<Ball>>,
        Query<&Transform, With<Paddle>>,
    )>,
    arena: Res<Arena>,
    mut score: ResMut<Score>,
) {
    let paddles: Vec<Transform> = set.p1().iter().copied().collect();

    for (mut transform, mut velocity) in set.p0().iter_mut() {
        // score if ball goes out of bounds and reset ball position
        if transform.translation.x + BALL_RADIUS >= arena.width / 2. {
            score.0 += 1;
            transform.translation = Vec3::new(0., 0., 0.);
            velocity.x = -velocity.x;
        } else if transform.translation.x - BALL_RADIUS <= -arena.width / 2. {
            transform.translation = Vec3::new(0., 0., 0.);
            velocity.x = -velocity.x;
        }

        // check for collision with paddles
        for paddle in paddles.iter() {
            if paddle_collision_check(&transform, paddle) {
                velocity.x = -velocity.x;
                velocity.y = (transform.translation.y - paddle.translation.y) * 5.;
            }
        }

        // check for collision with ceiling/floor
        if transform.translation.y + BALL_RADIUS >= arena.height / 2.
            || transform.translation.y - BALL_RADIUS <= -arena.height / 2.
        {
            velocity.y = -velocity.y;
        }

        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// TODO: make this more generic
fn paddle_collision_check(ball: &Transform, paddle: &Transform) -> bool {
    let ball_x = ball.translation.x;
    let ball_y = ball.translation.y;
    let ball_width = BALL_RADIUS * 2.;
    let ball_height = BALL_RADIUS * 2.;
    let paddle_x = paddle.translation.x;
    let paddle_y = paddle.translation.y;

    if ball_x + ball_width / 2. >= paddle_x - PADDLE_WIDTH / 2.
        && ball_x - ball_width / 2. <= paddle_x + PADDLE_WIDTH / 2.
        && ball_y + ball_height / 2. >= paddle_y - PADDLE_HEIGHT / 2.
        && ball_y - ball_height / 2. <= paddle_y + PADDLE_HEIGHT / 2.
    {
        return true;
    }
    false
}

fn move_paddle_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    arena: Res<Arena>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += 200. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 200. * time.delta_seconds();
        }
        if transform.translation.y + PADDLE_HEIGHT / 2. >= arena.height / 2. {
            transform.translation.y = arena.height / 2. - PADDLE_HEIGHT / 2.;
        }
        if transform.translation.y - PADDLE_HEIGHT / 2. <= -arena.height / 2. {
            transform.translation.y = -arena.height / 2. + PADDLE_HEIGHT / 2.;
        }
    }
}

fn score_text_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = score.0.to_string();
    }
}

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Arena {
            width: 0.,
            height: 0.,
            wall_thickness: 4.,
        });
        app.insert_resource(Score(0));
        app.add_systems(
            Startup,
            (
                setup_camera,
                setup_ball,
                (startup, setup_paddle, setup_arena, setup_score).chain(),
            ),
        );
        app.add_systems(
            Update,
            (
                score_text_update_system,
                ball_move_system,
                move_paddle_system,
            ),
        );
    }
}
