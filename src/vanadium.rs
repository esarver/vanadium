use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_core::prelude::*;

pub struct VanadiumPlugins;

impl Plugin for VanadiumPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin,
            ScheduleRunnerPlugin {
                run_mode: bevy_app::RunMode::Loop { wait: None },
            },
        ));
    }
}

