use std::{
    fmt::Display,
    io::{stdout, Stdout, Write},
};

use termion::{
    clear,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

pub struct App {
    stdout: AlternateScreen<RawTerminal<Stdout>>,
    width: u16,
    height: u16,
}

impl App {
    pub fn new() -> Self {
        let (width, height) = termion::terminal_size().unwrap();
        let mut app = App {
            stdout: AlternateScreen::from(stdout().into_raw_mode().unwrap()),
            width,
            height,
        };
        app.defer(clear::All);
        app.flush();
        app
    }

    fn defer(&mut self, operation: impl Display) {
        write!(self.stdout, "{}", operation).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}
