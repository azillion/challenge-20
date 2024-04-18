use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct Ball;

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

const X_EXTENT: f32 = 600.;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}

fn setup_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_mut();
    let window_width = window.width();
    let window_height = window.height();
    println!("window width: {}, height: {}", window_width, window_height);
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
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
        app.add_systems(
            Startup,
            (startup, setup_fps_counter, setup_camera, setup_paddles),
        );
        app.add_systems(Update, (fps_text_update_system, fps_counter_showhide));
        // .add_systems(Startup, add_people)
        // .add_systems(Update, (update_people, greet_people).chain());
    }
}

// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("Alex".to_string())));
//     commands.spawn((Person, Name("Ally".to_string())));
//     commands.spawn((Person, Name("Aether".to_string())));
// }

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// fn update_people(mut query: Query<&mut Name, With<Person>>) {
//     for mut name in &mut query {
//         if name.0 == "Alex" {
//             name.0 = "Bob".to_string();
//             break;
//         }
//     }
// }
