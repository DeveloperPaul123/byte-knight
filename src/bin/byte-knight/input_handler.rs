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
    sync::mpsc::{self, Receiver, Sender},
};

use uci_parser::UciCommand;

use crate::worker_thread::WorkerThread;

#[derive(Debug)]
pub(crate) struct InputHandler {
    worker: WorkerThread<UciCommand>,
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
    /// # Examples
    ///
    /// ```
    /// use engine::InputHandler;
    /// let input_handler = InputHandler::new();
    /// let receiver = input_handler.receiver();
    /// ```
    ///
    pub(crate) fn new() -> InputHandler {
        let (sender, receiver) = mpsc::channel();
        let worker = WorkerThread::new(sender.clone(), receiver, move |stop_flag| {
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
        InputHandler { worker }
    }

    fn sender(&self) -> Sender<UciCommand> {
        self.worker.sender()
    }

    /// Returns a reference to the receiver end of the channel.
    ///
    /// # Returns
    ///
    /// A reference to the receiver end of the channel.
    pub(crate) fn receiver(&self) -> &Receiver<UciCommand> {
        self.worker.receiver()
    }

    /// Signal to the worker thread that it should stop. This method does not block the calling
    /// thread.
    pub(crate) fn exit(&mut self) {
        self.worker.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_handler() {
        let input_handler = InputHandler::new();
        let sender = input_handler.sender();

        sender.send(UciCommand::Uci).unwrap();
        sender.send(UciCommand::IsReady).unwrap();
        sender.send(UciCommand::UciNewGame).unwrap();

        let receiver = input_handler.receiver();
        let inputs: Vec<UciCommand> = receiver.iter().take(3).collect();

        assert_eq!(
            inputs,
            vec![UciCommand::Uci, UciCommand::IsReady, UciCommand::UciNewGame]
        );
    }
}