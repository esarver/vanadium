use bevy_app::{prelude::*, ScheduleRunnerPlugin};
use bevy_core::prelude::*;

use crate::terminal;

pub struct VanadiumPlugins;

impl Plugin for VanadiumPlugins {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_plugins((
            TaskPoolPlugin::default(),
            TypeRegistrationPlugin,
            ScheduleRunnerPlugin {
                run_mode: bevy_app::RunMode::Loop { wait: None },
            },
        ));
    }
}

pub struct App {
    bevy_app: bevy_app::App,
}

impl Default for App {
    fn default() -> Self {
        let mut bevy_app = bevy_app::App::new();
        bevy_app.add_plugins((VanadiumPlugins, terminal::TerminalPlugin));
        Self { bevy_app }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        self.bevy_app.run();
    }
}
