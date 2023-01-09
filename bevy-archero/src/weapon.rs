use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub enum Weapon {
    #[default]
    Bow,
    Sword,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Weapon>()
            .register_type::<Damage>()
            .register_type::<Projectile>()
            .add_system(animate_projectile);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Reflect, Default)]
pub struct Damage(i32);

#[derive(Component, Reflect, Default)]
pub enum Projectile {
    #[default]
    Arrow,
    Fireball,
    Lightning,
}

fn animate_projectile(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
