use bevy::color::palettes::css::DARK_CYAN;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_prototype_lyon::prelude::*;

use crate::weapon::{Projectile, Velocity, Weapon, WeaponPlugin};

const ARCHER_SIZE: f32 = 30.0;

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

pub struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<Enemy>()
            .register_type::<Health>()
            .register_type::<Velocity>()
            .register_type::<Pose>()
            .register_type::<AnimationTimer>()
            .add_plugins((WeaponPlugin, ShapePlugin))
            .add_systems(Startup, add_player_and_enemy)
            .add_systems(
                Update,
                (
                    player_attack,
                    player_move,
                    player_animate,
                    enemy_move,
                    enemy_attack,
                ),
            );
    }
}

#[derive(Component, Reflect)]
struct AnimationTimer(Timer);

#[derive(Component, Reflect)]
struct AttackTimer(Timer);

fn player_animate(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlas, &Pose, &mut AnimationTimer)>,
) {
    for (mut sprite, pose, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
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

fn add_player_and_enemy(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/vimal-durai-archer-sprite-sheet.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        UVec2::new((960.0 / 6.0) as u32, (827.0 / 5.0) as u32),
        6,
        5,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_handle,
            ..default()
        },
        Player,
        Health(100),
        Weapon::Bow,
        Pose::Idle,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        Name::new("Player"),
    ));
    // let mesh: Handle<Mesh> = meshes.add(shape::Circle::default().into()).into();
    // let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    let random_translation = || {
        Vec3::new(
            rand::random::<f32>() * 1000.0 - 500.0,
            rand::random::<f32>() * 1000.0 - 500.0,
            0.0,
        )
    };
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(ARCHER_SIZE),
        ..shapes::RegularPolygon::default()
    };
    commands.spawn_batch((0..10).map(move |i| {
        (
            GeometryBuilder::build_as(&shape),
            Fill::color(DARK_CYAN),
            Stroke::new(Color::BLACK, 10.0),
            Transform {
                translation: random_translation(),
                ..default()
            },
            Enemy,
            Health(100),
            Weapon::Bow,
            Pose::Idle,
            AttackTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            Name::new(format!("Enemy {}", i)),
        )
    }));
}

// Add player control
fn player_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Pose, &mut Transform)>,
) {
    for (_, mut pose, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
            *pose = Pose::Walk;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
            *pose = Pose::Walk;
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
            *pose = Pose::Walk;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
            *pose = Pose::Walk;
        } else {
            *pose = Pose::Idle;
        }
        transform.translation += direction * 1.0;
    }
}

// Add player attack
fn player_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Weapon, &mut Pose, &Transform, &mut AttackTimer), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh: Handle<Mesh> = meshes.add(Circle::default()).into();
    let material = materials.add(Color::srgb(1.0, 0.5, 0.5));
    for (weapon, mut pose, transform, mut attack_timer) in query.iter_mut() {
        attack_timer.0.tick(time.delta());
        if !attack_timer.0.just_finished() {
            continue;
        }
        match weapon {
            Weapon::Bow => {
                *pose = Pose::Attack;
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(mesh.clone()),
                        material: material.clone(),
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::splat(ARCHER_SIZE / 2.0),
                            ..Default::default()
                        },
                        ..default()
                    },
                    Projectile::Arrow,
                    Velocity(Vec3::new(100.0, 100.0, 0.0)),
                ));
            }
            _ => todo!(),
        }
    }
}

fn enemy_move(mut query: Query<(&Enemy, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
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
        transform.translation += direction * 2.0;
    }
}

// Add player attack
fn enemy_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Weapon, &Transform, &mut AttackTimer), With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh: Handle<Mesh> = meshes.add(Circle::default()).into();
    let random_color = || {
        Color::srgb(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        )
    };
    let material = materials.add(random_color());
    for (weapon, transform, mut attack_timer) in query.iter_mut() {
        attack_timer.0.tick(time.delta());
        if !attack_timer.0.just_finished() {
            continue;
        }
        let random_velocity = || {
            Vec3::new(
                rand::random::<f32>() * 100.0 - 50.0,
                rand::random::<f32>() * 100.0 - 50.0,
                0.0,
            )
        };
        match weapon {
            Weapon::Bow => {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: bevy::sprite::Mesh2dHandle(mesh.clone()),
                        material: material.clone(),
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::splat(ARCHER_SIZE / 2.0),
                            ..default()
                        },
                        ..default()
                    },
                    Projectile::Arrow,
                    Velocity(random_velocity()),
                ));
            }
            _ => todo!(),
        }
    }
}
