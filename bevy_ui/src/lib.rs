mod anchors;
pub mod entity;
mod margins;
mod node;
mod ui_update_system;

pub use anchors::*;
pub use margins::*;
pub use node::*;
pub use ui_update_system::*;

use bevy_app::{AppBuilder, AppPlugin};

#[derive(Default)]
pub struct UiPlugin;

impl AppPlugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(ui_update_system());
    }
}