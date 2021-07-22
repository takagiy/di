use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};

use termion::{
    clear, cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

use crate::buffer;

pub struct App {
    stdout: StdoutExt,
    width: u16,
    height: u16,
    buffer: buffer::PieceTable,
}

struct StdoutExt(AlternateScreen<RawTerminal<Stdout>>);

impl StdoutExt {
    fn defer(&mut self, operation: impl Display) {
        write!(self.0, "{}", operation).unwrap();
    }

    fn flush(&mut self) {
        self.0.flush().unwrap();
    }
}

impl App {
    pub fn new() -> Self {
        let (width, height) = termion::terminal_size().unwrap();
        let mut app = App {
            stdout: StdoutExt(AlternateScreen::from(stdout().into_raw_mode().unwrap())),
            buffer: buffer::PieceTable::new(),
            width,
            height,
        };
        app.stdout.defer(clear::All);
        app.stdout.flush();
        app
    }

    pub fn run(&mut self) {
        {
        let mut cursor = self.buffer.cursor();
        cursor.insert("hello");
        cursor.insert(", world!");
        cursor.move_prev();
        cursor.move_prev();
        cursor.insert("XXXXXXXX");
        }
        {
        let mut cursor = self.buffer.cursor();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.insert("hello");
        cursor.insert(", world!");
        }
        self.draw();
    }

    fn draw(&mut self) {
        self.stdout.defer(clear::All);
        self.stdout.defer(cursor::Goto(1, 1));
        for c in self.buffer.iter() {
            self.stdout.defer(c);
        }
        self.stdout.flush();
    }
}
