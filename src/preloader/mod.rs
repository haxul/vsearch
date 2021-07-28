use std::{thread, io};
use std::time::Duration;
use std::sync::mpsc::{TryRecvError, Receiver};

pub fn start(rx: Receiver<()>) {
    thread::spawn(move || {
        let mut s = ".......";
        println!("start downloading...");
        loop {
            s = cycle_dot(s);
            println!("{}", s);
            thread::sleep(Duration::from_secs(1));
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(_) => {}
            }
        }
    });
}

fn cycle_dot(s: &str) -> &str {
    match s {
        "......." => "..|||..",
        "..|||.." => "..\\|/..",
        "..\\|/.." => ".......",
        _ => "......."
    }
}