use crate::display::Display;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use super::keyboard::get_keycode;

pub fn render(display: &Display) {
    let mut fmted_row = String::with_capacity(64);

    for row in display.buffer {
        for bit in row {
            match bit {
                0 => fmted_row += "\x1b[1;30m██",
                1 => fmted_row += "\x1b[1;37m██",
                _ => panic!("Bit can only be 0 or 1"),
            }
        }
        println!("{fmted_row}");
        fmted_row.clear();
    }
}

pub fn is_key_down(key: u8) -> bool {
    let stdin_channel = spawn_stdin_channel();
    if let Ok(ikey) = stdin_channel.try_recv()
    && key ==  get_keycode(ikey)
    {
        trace!("Pressed: {key}");
        true
    } else {
        false
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
