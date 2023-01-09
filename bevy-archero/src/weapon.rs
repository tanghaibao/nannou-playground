use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
enum Weapon {
    #[default]
    Bow,
    Sword,
}

#[derive(Component, Reflect, Default)]
struct Damage(i32);
