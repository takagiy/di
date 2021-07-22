use std::{
    fmt::Display,
    io::{stdin, stdout, Stdout, Write},
};

use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
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
        let mut cursor = self.buffer.cursor();
        for event in stdin().keys() {
            use Key::*;
            match event.unwrap() {
                Ctrl('c') => break,
                Left => cursor.move_prev(),
                Right => cursor.move_next(),
                Char(c) => {
                    let mut buf = [0u8; 4];
                    cursor.insert(c.encode_utf8(&mut buf));
                }
                _ => (),
            }
            Self::draw(&mut self.stdout, cursor.iter());
        }
    }

    fn draw(stdout: &mut StdoutExt, chars: impl Iterator<Item = char>) {
        stdout.defer(clear::All);
        stdout.defer(cursor::Goto(1, 1));
        for c in chars {
            stdout.defer(c);
        }
        stdout.flush();
    }
}
