use std::{fs, io::stdout, path::PathBuf};

use clap::Parser;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use thiserror::Error;

#[derive(Error, Debug)]
enum VanadiumErrors {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file to open in the editor
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let contents = fs::read_to_string(cli.file)?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    'app_loop: loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(contents.clone()), area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break 'app_loop;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
