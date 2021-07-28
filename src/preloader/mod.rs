use std::{thread, io};
use std::time::Duration;
use std::sync::mpsc::{TryRecvError, Receiver};
use std::io::{stdout, Write};

pub fn start(rx: Receiver<()>) {
    thread::spawn(move || {
        println!("downloading....");
        loop {
            thread::sleep(Duration::from_secs(1));
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(_) => {}
            }
        }
    });
}