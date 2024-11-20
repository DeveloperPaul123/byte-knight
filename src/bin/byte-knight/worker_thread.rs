use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::JoinHandle,
};

/// WorkerThread is a wrapper around a thread that runs a worker function. The worker function is
/// passed to the constructor and is executed in a loop until the worker thread is stopped.
/// 
/// The worker function is passed an `Arc<AtomicBool>` that can be used to check if the worker
/// thread should stop.
///
#[derive(Debug)]
pub(crate) struct WorkerThread<T: Send + 'static> {
    handle: Option<JoinHandle<()>>,
    sender: Sender<T>,
    receiver: Receiver<T>,
    stop_flag: Arc<AtomicBool>,
}

impl<T: Send + 'static> WorkerThread<T> {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn new<F>(sender: Sender<T>, receiver: Receiver<T>, worker: F) -> WorkerThread<T>
    where
        F: Fn(Arc<AtomicBool>) + Send + 'static,
    {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = stop_flag.clone();
        let handle = Some(std::thread::spawn(move || {
            worker(stop_flag_clone.clone());
        }));

        WorkerThread {
            handle,
            sender,
            receiver,
            stop_flag,
        }
    }

    pub fn send(&self, data: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(data)
    }

    pub fn sender(&self) -> Sender<T> {
        self.sender.clone()
    }

    pub fn receiver(&self) -> &Receiver<T> {
        &self.receiver
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);

        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

impl<T: Send + 'static> Drop for WorkerThread<T> {
    fn drop(&mut self) {
        self.stop();
    }
}
