use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;
const BALL_RADIUS: f32 = 7.0;

pub struct PongPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PongPlugin,
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
struct Score {
    player1: usize,
    player2: usize,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Player2;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Wall;

fn startup(windows: Query<&Window>, mut arena: ResMut<Arena>) {
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();
    let wall_thickness = 4.;
    let mut arena_width = (window_width - wall_thickness * 2.) * 0.9;
    let mut arena_height = (window_height - wall_thickness * 2.) * 0.8;
    if arena_width / arena_height > 2. {
        arena_width = arena_height * 2.;
    } else {
        arena_height = arena_width / 2.;
    }
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
        Wall,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.width, arena.wall_thickness))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., arena.height / 2. + arena.wall_thickness / 2., 0.),
            ..Default::default()
        },
        Position {
            x: 0.,
            y: arena.height / 2. + arena.wall_thickness / 2.,
        },
    ));

    // Bottom Wall
    commands.spawn((
        Wall,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.width, arena.wall_thickness))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., -arena.height / 2. - arena.wall_thickness / 2., 0.),
            ..Default::default()
        },
        Position {
            x: 0.,
            y: -arena.height / 2. - arena.wall_thickness / 2.,
        },
    ));

    // Left Wall
    commands.spawn((
        Wall,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.wall_thickness, arena.height))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-arena.width / 2. - arena.wall_thickness / 2., 0., 0.),
            ..Default::default()
        },
        Position {
            x: -arena.width / 2. - arena.wall_thickness / 2.,
            y: 0.,
        },
    ));

    // Right Wall
    commands.spawn((
        Wall,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(arena.wall_thickness, arena.height))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(arena.width / 2. + arena.wall_thickness / 2., 0., 0.),
            ..Default::default()
        },
        Position {
            x: arena.width / 2. + arena.wall_thickness / 2.,
            y: 0.,
        },
    ));
}

fn setup_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    arena: Res<Arena>,
) {
    let paddle_padding = 10.;
    commands.spawn((
        Paddle,
        Player1,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                (-arena.width / 2. + PADDLE_WIDTH / 2.) + paddle_padding,
                0.,
                0.,
            ),
            ..Default::default()
        },
        Position {
            x: (-arena.width / 2. + PADDLE_WIDTH / 2.) + paddle_padding,
            y: 0.,
        },
        Velocity { x: 0., y: 0. },
    ));
    commands.spawn((
        Paddle,
        Player2,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                (arena.width / 2. - PADDLE_WIDTH / 2.) - paddle_padding,
                0.,
                0.,
            ),
            ..Default::default()
        },
        Position {
            x: (arena.width / 2. - PADDLE_WIDTH / 2.) - paddle_padding,
            y: 0.,
        },
        Velocity { x: 0., y: 0. },
    ));
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Ball,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BALL_RADIUS, BALL_RADIUS))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Position { x: 0., y: 0. },
        Velocity { x: 0., y: 0. },
    ));
}

fn setup_score(mut commands: Commands, arena: Res<Arena>) {
    commands.spawn((
        Player1,
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
    commands.spawn((
        Player2,
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
                right: Val::Px(arena.width / 2. - 50.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

fn score_text_update_system(
    mut queries: ParamSet<(
        Query<&mut Text, (With<Player1>, With<ScoreText>)>,
        Query<&mut Text, (With<Player2>, With<ScoreText>)>,
    )>,
    score: Res<Score>,
) {
    for mut text in queries.p0().iter_mut() {
        text.sections[0].value = score.player1.to_string();
    }
    for mut text in queries.p1().iter_mut() {
        text.sections[0].value = score.player2.to_string();
    }
}

fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Arena {
            width: 0.,
            height: 0.,
            wall_thickness: 4.,
        });
        app.insert_resource(Score {
            player1: 0,
            player2: 0,
        });
        app.add_systems(
            Startup,
            (
                setup_fps_counter,
                setup_camera,
                setup_ball,
                (startup, setup_paddles, setup_arena, setup_score).chain(),
            ),
        );
        app.add_systems(
            Update,
            (
                fps_text_update_system,
                fps_counter_showhide,
                score_text_update_system,
            ),
        );
    }
}
