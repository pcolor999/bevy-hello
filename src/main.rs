use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Codespace!".into(),
                // Fit the canvas to the web browser window
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Add a 2D Camera
    commands.spawn(Camera2d);

    // Add a red square
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.2, 0.2),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
    ));
}

// fn main() {
//     println!("Hello, world!");
// }
