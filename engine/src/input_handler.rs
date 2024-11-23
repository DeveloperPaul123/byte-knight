/*
 * input_handler.rs
 * Part of the byte-knight project
 * Created Date: Monday, November 18th 2024
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * -----
 * Last Modified: Thu Nov 21 2024
 * -----
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 *
 */

use std::{
    io::{stdin, BufRead},
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
    thread::JoinHandle,
};

use uci_parser::UciCommand;

#[derive(Debug)]
pub(crate) struct InputHandler {
    handle: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    receiver: Receiver<UciCommand>,
}

impl InputHandler {
    /// Creates a new [`InputHandler`]. The input handler reads from stdin and sends the parsed UCI
    /// commands to the receiver end of the channel via the sender. Creating a new [`InputHandler`]
    /// spawns a new worker thread. The thread starts upon creation.
    ///
    /// # Panics
    ///
    /// Panics if there is an error spawning the worker thread.
    ///
    /// # Returns
    ///
    /// A new [`InputHandler`] instance.
    ///
    pub(crate) fn new() -> InputHandler {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = stop_flag.clone();
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let stdin = stdin();
            let mut input = stdin.lock().lines();
            while !stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                if let Some(Ok(line)) = input.next() {
                    let command = UciCommand::from_str(line.as_str());
                    if let Ok(command) = command {
                        let cmd = command.clone();
                        sender.send(command).unwrap();
                        // manually break the loop if the command is "quit"
                        if cmd == UciCommand::Quit {
                            break;
                        }
                    } else {
                        eprintln!("Invalid UCI command: {}", line);
                    }
                } else {
                    eprintln!("Error reading from stdin");
                }
            }
        });
        InputHandler {
            handle: Some(worker),
            stop_flag: stop_flag_clone,
            receiver,
        }
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }

    pub fn receiver(&self) -> &Receiver<UciCommand> {
        &self.receiver
    }

    /// Signal to the worker thread that it should stop. This method does not block the calling
    /// thread.
    pub(crate) fn exit(&mut self) {
        self.stop();
    }
}
