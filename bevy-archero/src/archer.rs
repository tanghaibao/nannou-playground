use bevy::prelude::*;
// use bevy::sprite::MaterialMesh2dBundle;
use bevy::log::info;

// const ARCHER_SIZE: f32 = 30.0;

#[derive(Component, Reflect, Default)]
struct Archer;

#[derive(Component, Reflect, Default)]
struct Player;

#[derive(Component, Reflect, Default)]
struct Enemy;

#[derive(Component, Reflect, Default)]
struct Health(i32);

#[derive(Component, Reflect, Default)]
struct Velocity(Vec3);

pub struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archer>()
            .add_startup_system(add_archer)
            .add_system(player_control)
            .add_system(animate_archer);
    }
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_archer(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer)>,
) {
    for (mut sprite, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            info!("sprite index: {}", sprite.index);
            sprite.index = (sprite.index - 6 + 1) % 6 + 6;
        }
    }
}

fn add_archer(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("textures/vimal-durai-archer-sprite-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(960.0 / 6.0, 827.0 / 5.0),
        6,
        5,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        Archer,
        Player,
        Health(100),
        Velocity(Vec3::ZERO),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Name::new("Player"),
    ));
    // let mesh = meshes.add(shape::Circle::default().into()).into();
    // let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    // let transform = Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(ARCHER_SIZE));
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh,
    //         material,
    //         transform,
    //         ..default()
    //     },
    //     Archer,
    //     Player,
    //     HitPoints(100),
    //     Velocity(Vec3::ZERO),
    // ));
}

// Add player control
fn player_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        transform.translation += direction * 1.0;
    }
}
