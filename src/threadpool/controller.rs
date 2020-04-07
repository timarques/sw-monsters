use crate::threadpool::thread::Thread;
use std::sync::{Arc, Mutex};

pub(crate) type Callback = Box<dyn FnOnce() + Send + 'static>;

pub(crate) struct Work {
    pub priority: i32,
    pub tasks: Vec<Callback>
}

pub(crate) struct Controller {
    threads: Vec<Thread>,
    elements: Vec<Work>,
    workers: i32
}

impl Controller {

    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            threads: Vec::new(),
            elements: Vec::new(),
            workers: 0
        }))
    }

    pub fn ready(&mut self) {
        self.try_sync_number_of_threads(None);
        self.delegate_new_task();
    }

    pub fn work(&mut self, priority: i32) -> usize {
        let work = Work {priority, tasks: Vec::new()};
        let mut iterator = self.elements.iter();

        /*if let Some(position) = match lifo {
            true => iterator.position(|saved_work| work.priority >= saved_work.priority).or(Some(0)),
            false => iterator.position(|saved_work| saved_work.priority > work.priority)
        } {
            self.elements.insert(position, work);
        } else {
            self.elements.push(work);
        }*/

        if let Some(position) = iterator.position(|saved_work| saved_work.priority > work.priority) {
            self.elements.insert(position, work);
        } else {
            self.elements.push(work);
        }
        self.delegate_new_task();
        self.elements.len() - 1
    }

    pub fn task(&mut self, callback: Callback, lifo: bool, position: usize) {
        if let Some(work) = self.elements.get_mut(position) {
            if lifo {
                work.tasks.insert(0, callback);
            } else {
                work.tasks.push(callback);
            }
            self.delegate_new_task();
        }
    }

    /*pub fn work(&mut self, work: Work, lifo: bool) -> i32 {
        let length = self.work_elements.len();
        let mut iterator = self.work_elements.iter();
        if let Some(position) = match lifo {
            true => iterator.position(|saved_work| work.priority >= saved_work.priority).or(Some(0)),
            false => iterator.position(|saved_work| saved_work.priority > work.priority)
        } {
            self.work_elements.insert(position, work);
        } else {
            self.work_elements.push(work);
        }
        self.delegate_new_task();
        length as i32
    }*/

    /*pub fn task(&mut self, task: Task, position: i32, lifo: bool) -> i32 {
        let work = &mut self.work_elements[position as usize];
        let length = work.tasks.len();
        let mut iterator = work.tasks.iter();
        if let Some(position) = match lifo {
            true => iterator.position(|saved_task| task.priority >= saved_task.priority).or(Some(0)),
            false => iterator.position(|saved_task| saved_task.priority > task.priority)
        } {
            work.tasks.insert(position, task);
        } else {
            work.tasks.push(task);
        }
        self.delegate_new_task();
        length as i32
    }*/

    pub fn try_sync_number_of_threads(&mut self, workers: Option<i32>) {
        self.workers = workers.unwrap_or(self.workers);
        let mut diff = self.workers - self.threads.len() as i32;

        if diff < 0 {
            for (index, thread) in self.threads.clone().iter().enumerate() {
                if !thread.is_busy() {
                    let thread = self.threads.remove(index);
                    thread.kill();
                    diff += 1;
                }
            }
        } else if diff > 0 {
            while diff > 0 {
                self.threads.push(Thread::new());
                self.delegate_new_task();
                diff -=  1;
            }
        }
    }

    fn free_thread(&self) -> Option<Thread> {
        self.threads.iter().find_map(|thread| {
            match thread.is_busy() {
                true => None,
                false => Some(thread.clone())
            }
        })
    }

    fn delegate_new_task(&mut self) {
        let thread = self.free_thread();
        if self.elements.len() > 0 && thread.is_some() {
            for element in &mut self.elements {
                if element.tasks.len() > 0 {
                    thread.as_ref().unwrap().execute(element.tasks.remove(0));
                    break;
                }
            }
        }
    }

}
