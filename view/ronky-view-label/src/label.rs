use bevy::prelude::*;

#[derive(Component)]
pub struct Label;

pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    fn build(&self, app: &mut App) {}
}
