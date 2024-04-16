use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things
        app
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people)).chain());
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alex".to_string())));
    commands.spawn((Person, Name("Ally".to_string())));
    commands.spawn((Person, Name("Aether".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Alex" {
            name.0 = "Bob".to_string();
            break;
        }
    }
}

fn hello_world() {
    println!("hello world!");
}
