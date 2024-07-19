use bevy_app::Plugin;
use bevy_ecs::prelude::*;

#[derive(Debug)]
struct AutoTile;

impl Plugin for AutoTile {
    fn build(&self, app: &mut bevy_app::App) {}
}

fn layout_buffers() {}
