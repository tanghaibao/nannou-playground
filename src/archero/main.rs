use bevy::color::palettes::css::GOLD;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
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
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            WorldInspectorPlugin::default(),
            ArcherPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (text_color_system, text_update_system))
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
        )
        // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
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
                    color: GOLD.into(),
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
    let red = (1.25 * seconds).sin() / 2.0 + 0.5;
    let green = (0.75 * seconds).sin() / 2.0 + 0.5;
    let blue = (0.50 * seconds).sin() / 2.0 + 0.5;
    let alpha = 1.0;
    text.sections[0].style.color = Color::Srgba(Srgba {
        red,
        green,
        blue,
        alpha,
    });
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
