use bevy::prelude::*;

pub struct Configuration;

impl Plugin for Configuration {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
