use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod archer;
use archer::ArcherPlugin;

mod weapon;

#[derive(Component, Reflect, Default)]
pub struct ColorText;

#[derive(Component, Reflect, Default)]
struct FpsText;

const FONT_SIZE: f32 = 30.0;

fn main() {
    App::new()
        .register_type::<ColorText>()
        .register_type::<FpsText>()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(ArcherPlugin)
        .add_startup_system(setup)
        .add_system(text_update_system)
        .add_system(text_color_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "bevy",
            TextStyle {
                font: font.clone(),
                font_size: FONT_SIZE,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
        Name::new("bevy"),
    ));
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font,
                    font_size: FONT_SIZE,
                    color: Color::GOLD,
                },
            ),
        ]),
        FpsText,
        Name::new("fps"),
    ));
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    let mut text = query.single_mut();
    let seconds = time.elapsed_seconds();

    // Update the color of the first and only section.
    text.sections[0].style.color = Color::Rgba {
        red: (1.25 * seconds).sin() / 2.0 + 0.5,
        green: (0.75 * seconds).sin() / 2.0 + 0.5,
        blue: (0.50 * seconds).sin() / 2.0 + 0.5,
        alpha: 1.0,
    };
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
