mod thread;
mod controller;

use std::sync::{Mutex, Arc};
use std::sync::atomic::{Ordering, AtomicBool};

pub struct ThreadPool {
    lifo: AtomicBool,
    controller: Arc<Mutex<controller::Controller>>,
    work_position: usize
}

impl Clone for ThreadPool {

    fn clone(&self) -> Self {
        Self {
            lifo: AtomicBool::new(self.lifo.load(Ordering::Relaxed)),
            controller: self.controller.clone(),
            work_position: self.work_position
        }
    }

}

impl ThreadPool {

    pub fn new() -> Self {
        let controller = controller::Controller::new();
        let work_position = controller.lock().unwrap().work(0);
        Self {
            lifo: AtomicBool::new(false),
            controller,
            work_position
        }
    }

    pub fn add<A:  FnOnce() + Send + 'static>(&self, callback: A) -> &Self {
        let controller = self.controller.clone();
        self.controller.lock().unwrap().task(Box::new(move || {
            callback();
            controller.lock().unwrap().ready();
        }), self.lifo.load(Ordering::Relaxed), self.work_position);
        self
    }

    pub fn group(&self, priority: i32) -> Self {
        let lifo = self.lifo.load(Ordering::Relaxed);
        Self {
            controller: self.controller.clone(),
            work_position: self.controller.lock().unwrap().work(priority),
            lifo: AtomicBool::new(lifo)
        }
    }

    //Will be LIFO when have same priority
    pub fn lifo(&self, lifo: bool) -> &Self {
        self.lifo.store(lifo, Ordering::Relaxed);
        self
    }

    pub fn workers(&self, workers: i32) -> &Self {
        self.controller.lock().unwrap().try_sync_number_of_threads(Some(workers));
        self
    }

}
