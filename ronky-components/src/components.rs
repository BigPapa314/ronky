use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub enum Unit {
    Milimeters,
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Miliseconds,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Value(f32);

#[derive(Component)]
pub struct Max(f32);

#[derive(Component)]
pub struct Min(f32);

#[derive(Component)]
pub struct History(Vec<f32>);
