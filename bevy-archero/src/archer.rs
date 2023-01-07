use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const ARCHER_SIZE: f32 = 30.0;

#[derive(Component, Reflect, Default)]
struct Archer;

#[derive(Component, Reflect, Default)]
enum Weapon {
    #[default]
    Bow,
    Sword,
}

#[derive(Component, Reflect, Default)]
struct HitPoints(i32);

pub struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archer>().add_startup_system(add_archer);
    }
}

fn add_archer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(shape::Circle::default().into()).into();
    let material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    let transform = Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(ARCHER_SIZE));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        },
        Archer,
        HitPoints(100),
    ));
}
