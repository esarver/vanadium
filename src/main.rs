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

enum Mode {
    Normal,
    Insert,
    Replace,
    Command,
}

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn move_up(&mut self, n: u16) {
        self.y = self.y.saturating_sub(n);
    }

    fn move_down(&mut self, n: u16) {
        self.y = self.y.saturating_add(n);
    }

    fn move_right(&mut self, n: u16) {
        self.x = self.x.saturating_add(n);
    }

    fn move_left(&mut self, n: u16) {
        self.x = self.x.saturating_sub(n);
    }
}

struct App {
    pub cursor: Cursor,
    pub mode: Mode,
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let contents = fs::read_to_string(cli.file)?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App {
        cursor: Cursor { x: 0, y: 0 },
        mode: Mode::Normal,
    };

    'app_loop: loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new(contents.clone()), area);
            frame.set_cursor(app.cursor.x, app.cursor.y);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Normal if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char('q') => break 'app_loop,
                        KeyCode::Char('k') => app.cursor.move_up(1),
                        KeyCode::Char('j') => app.cursor.move_down(1),
                        KeyCode::Char('l') => app.cursor.move_right(1),
                        KeyCode::Char('h') => app.cursor.move_left(1),
                        KeyCode::Char('i') => app.mode = Mode::Insert,
                        KeyCode::Char('r') => app.mode = Mode::Replace,
                        KeyCode::Char(':') => app.mode = Mode::Command,
                        _ => {}
                    },
                    Mode::Insert if key.kind == KeyEventKind::Press => {}
                    Mode::Replace if key.kind == KeyEventKind::Press => {}
                    Mode::Command if key.kind == KeyEventKind::Press => {}
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
