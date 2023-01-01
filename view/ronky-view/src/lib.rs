use bevy::prelude::*;

pub mod prelude;

use ronky_view_label::label::LabelPlugin;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LabelPlugin);
    }
}
