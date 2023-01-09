use bevy::log::info;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const ARCHER_SIZE: f32 = 30.0;

#[derive(Component, Reflect, Default)]
struct Archer;

#[derive(Component, Reflect, Default)]
struct Player;

#[derive(Component, Reflect, Default)]
struct Enemy;

#[derive(Component, Reflect, Default)]
struct Health(i32);

#[derive(Component, Reflect, Default, PartialEq, Eq, Hash)]
enum Pose {
    #[default]
    Idle,
    Walk,
    Attack,
    Die,
}

#[derive(Component, Reflect, Default)]
struct Velocity(Vec3);

pub struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archer>()
            .register_type::<Player>()
            .register_type::<Enemy>()
            .register_type::<Health>()
            .register_type::<Velocity>()
            .register_type::<AnimationTimer>()
            .add_startup_system(add_archer)
            .add_system(player_control)
            .add_system(animate_player)
            .add_system(enemy_move);
    }
}

#[derive(Component, Reflect)]
struct AnimationTimer(Timer);

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &Pose, &mut AnimationTimer)>,
) {
    for (mut sprite, pose, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // info!("sprite index: {}", sprite.index);
            match pose {
                Pose::Idle => {
                    sprite.index = 1;
                }
                Pose::Walk => {
                    sprite.index = (sprite.index - 6 + 1) % 6 + 6;
                }
                Pose::Attack => {
                    sprite.index = (sprite.index - 12 + 1) % 6 + 12;
                }
                Pose::Die => {
                    sprite.index = (sprite.index - 18 + 1) % 6 + 18;
                }
            }
        }
    }
}

fn add_archer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
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
        Pose::Idle,
        Velocity(Vec3::ZERO),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Name::new("Player"),
    ));
    let mesh: Handle<Mesh> = meshes.add(shape::Circle::default().into()).into();
    let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    let random_translation = || {
        Vec3::new(
            rand::random::<f32>() * 1000.0 - 500.0,
            rand::random::<f32>() * 1000.0 - 500.0,
            0.0,
        )
    };
    commands.spawn_batch((0..10).map(move |i| {
        (
            MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(mesh.clone()),
                material: material.clone(),
                transform: Transform {
                    translation: random_translation(),
                    scale: Vec3::splat(ARCHER_SIZE),
                    ..Default::default()
                },
                ..default()
            },
            Archer,
            Enemy,
            Health(100),
            Pose::Idle,
            Velocity(Vec3::splat(6.0)),
            Name::new(format!("Enemy {}", i)),
        )
    }));
}

// Add player control
fn player_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Pose, &mut Transform)>,
) {
    for (_, mut pose, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
            *pose = Pose::Walk;
        } else if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
            *pose = Pose::Walk;
        } else if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
            *pose = Pose::Walk;
        } else if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
            *pose = Pose::Walk;
        } else {
            *pose = Pose::Idle;
        }
        transform.translation += direction * 1.0;
    }
}

fn enemy_move(time: Res<Time>, mut query: Query<(&Enemy, &mut Transform, &mut Velocity)>) {
    for (_, mut transform, mut velocity) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if rand::random::<f32>() < 0.1 {
            direction.x += 1.0;
        }
        if rand::random::<f32>() < 0.1 {
            direction.x -= 1.0;
        }
        if rand::random::<f32>() < 0.1 {
            direction.y += 1.0;
        }
        if rand::random::<f32>() < 0.1 {
            direction.y -= 1.0;
        }
        transform.translation += direction * 1.0;
    }
}