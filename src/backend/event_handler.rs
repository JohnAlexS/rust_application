use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use log::error;
use std::thread;
use std::sync::mpsc;

use crate::backend::key_map::{Key as bKey};

pub enum Ticks{
    Input(bKey),
    Tick
}

pub struct EventHandler{
    rx : mpsc::Receiver<Ticks>,
    _tx : mpsc::Sender<Ticks>,
    kill_thread : Arc<AtomicBool>
}

impl EventHandler{
    pub fn new_handler(poll_time: Duration) -> EventHandler{
        let (tx, rx) = mpsc::channel();
        let kill_thread = Arc::new(AtomicBool::new(false));

        let event_tx = tx.clone();
        let event_kill_thread = kill_thread.clone();

        thread::spawn(move || {
            loop{
                if crossterm::event::poll(poll_time).unwrap() {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap(){
                        let key: Ticks = Ticks::Input(bKey::from(key));
                        if let Err(_e) = event_tx.send(key){
                            error!("stupid ass");
                        }
                    }
                    else{
                        if let Err(_e) = event_tx.send(Ticks::Tick){
                            error!("stupid ass");
                        }
                    }
                }
                if event_kill_thread.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        EventHandler{
            rx,
            _tx : tx,
            kill_thread
        }
    }

    pub async fn read_next(&mut self) -> Ticks{
        self.rx.recv().unwrap()
    }

    pub fn end(&mut self) {
        self.kill_thread.store(true, Ordering::Relaxed);
    }
}