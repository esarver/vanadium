use bevy_app::prelude::*;

pub mod terminal;

pub mod vanadium {
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
}

fn main() -> anyhow::Result<()> {
    App::new()
        .add_plugins(vanadium::VanadiumPlugins)
        .add_plugins(terminal::TerminalPlugin)
        .run();

    Ok(())
}
