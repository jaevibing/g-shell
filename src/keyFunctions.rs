use std::{io::{stdin, Write}, sync::Arc};
use termion::{event::Key, input::TermRead, raw::RawTerminal};

pub fn getKeyPresses(termout: Arc<std::sync::Mutex<RawTerminal<std::io::Stdout>>>){
    let mut termout_lock = termout.lock().unwrap();
    loop{
        let termin = stdin();
        for c in termin.keys() {
            match c.unwrap() {
                Key::Left => write!(termout_lock, "{}", termion::cursor::Left(1)),
                Key::Right => write!(termout_lock, "{}", termion::cursor::Right(1)),
                _ => todo!(),
            };
        }
    }
}