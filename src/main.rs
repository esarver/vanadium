use bevy_app::prelude::*;

pub mod terminal;
pub mod vanadium;

fn main() -> anyhow::Result<()> {
    App::new()
        .add_plugins(vanadium::VanadiumPlugins)
        .add_plugins(terminal::TerminalPlugin)
        .run();

    Ok(())
}
