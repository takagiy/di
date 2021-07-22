#![feature(linked_list_cursors)]
mod app;
mod buffer;
use std::{thread::sleep, time::Duration};

use app::*;

fn main() {
    let mut app = App::new();
    app.run();
    sleep(Duration::from_secs(1));
}
