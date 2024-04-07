use bevy_app::{prelude::*, AppExit};
use bevy_ecs::prelude::*;
use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::CrosstermBackend;
use std::io::stdout;

#[derive(Default)]
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, start_terminal)
            .add_systems(PreUpdate, poll_input_keys);
    }
}

#[derive(Component)]
pub struct Terminal<B>
where
    B: ratatui::backend::Backend,
{
    terminal: ratatui::Terminal<B>,
}

impl<B> Drop for Terminal<B>
where
    B: ratatui::backend::Backend,
{
    fn drop(&mut self) {
        stdout()
            .execute(LeaveAlternateScreen)
            .expect("should be able to leave alternate screen");
        disable_raw_mode().expect("should be able to disable raw mode");
    }
}

pub fn start_terminal(mut cmds: Commands) {
    stdout()
        .execute(EnterAlternateScreen)
        .expect("should be able to enter alternate screen");
    enable_raw_mode().expect("should be able to enable raw mode");
    let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))
        .expect("should be able to create a new terminal");
    terminal
        .clear()
        .expect("should be able to clear the terminal");

    cmds.spawn(Terminal { terminal });
}

pub fn poll_input_keys(mut event_writer: EventWriter<AppExit>) {
    if crossterm::event::poll(std::time::Duration::from_millis(16)).unwrap() {
        if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    event_writer.send(AppExit);
                }
                _ => {}
            };
        }
    }
}
