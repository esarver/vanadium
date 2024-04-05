use std::{fs, io::stdout, path::PathBuf};

use bevy_ecs::world::World;
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

#[derive(Default)]
struct Vanadium {
    world: World,
}

impl Vanadium {
    //TODO: Keep cursor position, but clamp location to end of line
//    fn move_cursor_up(&mut self, n: u16) {
//        let num_lines = self.viewer.document.contents.len();
//        self.cursor.move_up(n, num_lines as u16);
//        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
//        self.cursor.move_right(0, line_len as u16);
//    }
//    fn move_cursor_down(&mut self, n: u16) {
//        let num_lines = self.viewer.document.contents.len();
//        self.cursor.move_down(n, num_lines as u16);
//        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
//        self.cursor.move_right(0, line_len as u16);
//    }
//    fn move_cursor_right(&mut self, n: u16) {
//        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
//        self.cursor.move_right(n, line_len as u16);
//    }
//    fn move_cursor_left(&mut self, n: u16) {
//        let line_len = self.viewer.document.line_length(self.cursor.y as usize);
//        self.cursor.move_left(n, line_len as u16);
//    }

    fn run(self) -> anyhow::Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;
        //'app_loop: loop {
        //    terminal.draw(|frame| {
        //        let _area = frame.size();
        //    })?;

        //    if event::poll(std::time::Duration::from_millis(16))? {
        //        if let event::Event::Key(key) = event::read()? {
        //        }
        //    }
        //}
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let contents = fs::read_to_string(cli.file)?;

    let _document: Document = Document {
        contents: contents.lines().collect(),
    };
    let app = Vanadium::default();

    app.run()?;

    Ok(())
}
