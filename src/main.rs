use std::{fs, io::stdout, path::PathBuf};

use clap::Parser;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    text::{Line, Text},
    Terminal,
};
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
    fn move_up(&mut self, n: u16, max_y: u16) {
        self.y = self.y.saturating_sub(n).clamp(0, max_y);
    }

    fn move_down(&mut self, n: u16, max_y: u16) {
        self.y = self.y.saturating_add(n).clamp(0, max_y);
    }

    fn move_right(&mut self, n: u16, max_x: u16) {
        self.x = self.x.saturating_add(n).clamp(0, max_x);
    }

    fn move_left(&mut self, n: u16, max_x: u16) {
        self.x = self.x.saturating_sub(n).clamp(0, max_x);
    }
}

struct Document<'a> {
    pub contents: Vec<&'a str>,
}

impl Document<'_> {
    pub fn line_length(&self, line_number: usize) -> usize {
        if let Some(line) = self.contents.get(line_number) {
            line.len()
        } else {
            0
        }
    }
}

struct DocumentViewer<'a> {
    pub document: &'a Document<'a>,
}

impl DocumentViewer<'_> {
    fn get_text(&self) -> Text {
        let mut text = Text::default();
        for line in self.document.contents.clone().into_iter() {
            text.lines.push(Line::raw(line));
        }
        text
    }
}

struct App<'a> {
    pub cursor: Cursor,
    pub mode: Mode,
    pub viewer: DocumentViewer<'a>,
}

impl App<'_> {
    //TODO: Keep cursor position, but clamp location to end of line
    fn move_cursor_up(&mut self, n: u16) {
        let num_lines = self.viewer.document.contents.len();
        self.cursor.move_up(n, num_lines as u16);
        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
        self.cursor.move_right(0, line_len as u16);
    }
    fn move_cursor_down(&mut self, n: u16) {
        let num_lines = self.viewer.document.contents.len();
        self.cursor.move_down(n, num_lines as u16);
        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
        self.cursor.move_right(0, line_len as u16);
    }
    fn move_cursor_right(&mut self, n: u16) {
        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
        self.cursor.move_right(n, line_len as u16);
    }
    fn move_cursor_left(&mut self, n: u16) {
        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
        self.cursor.move_left(n, line_len as u16);
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let contents = fs::read_to_string(cli.file)?;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let document: Document = Document {
        contents: contents.lines().collect(),
    };
    let mut app = App {
        cursor: Cursor { x: 0, y: 0 },
        mode: Mode::Normal,
        viewer: DocumentViewer {
            document: &document,
        },
    };

    'app_loop: loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(app.viewer.get_text(), area);
            // frame.render_widget(Paragraph::new(contents.clone()), area);
            frame.set_cursor(app.cursor.x, app.cursor.y);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Normal if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char('q') => break 'app_loop,
                        KeyCode::Char('k') => app.move_cursor_up(1),
                        KeyCode::Char('j') => app.move_cursor_down(1),
                        KeyCode::Char('l') => app.move_cursor_right(1),
                        KeyCode::Char('h') => app.move_cursor_left(1),
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
