use std::sync::{Mutex, Arc};
use std::sync::mpsc::{self, SyncSender, Receiver};
use std::sync::atomic::{Ordering, AtomicBool};

enum Message {
    Kill(),
    Execute(Box<dyn FnOnce() + Send + 'static>)
}

struct Checker<'a> {
    thread: &'a Thread
}

impl <'a> Checker <'a> {
    pub fn new(thread: &'a Thread) -> Self {
        Self {thread}
    }

}

impl Drop for Checker<'_> {
    fn drop(&mut self) {
        if std::thread::panicking(){
            Thread::init(self.thread.clone());
        }
    }
}

#[derive(Clone)]
pub(crate) struct Thread {
    busy: Arc<AtomicBool>,
    sender: SyncSender<Message>,
    receiver: Arc<Mutex<Receiver<Message>>>
}

impl Thread {

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::sync_channel(1);

        let thread = Self{
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            busy: Arc::new(AtomicBool::new(false))
        };
        Self::init(thread.clone());
        thread
    }

    pub fn init(this: Self) {
        std::thread::spawn(move || {
            let _checker = Checker::new(&this);
            loop {
                match this.receiver.lock().unwrap().recv().unwrap() {
                    Message::Kill() => break,
                    Message::Execute(callback) => {
                        this.busy.store(true, Ordering::Relaxed);
                        callback();
                        this.busy.store(false, Ordering::Relaxed);
                    }
                }
            }
        });
    }

    pub fn is_busy(&self) -> bool {
        self.busy.load(Ordering::Relaxed)
    }

    pub fn kill(&self) {
        self.sender.send(Message::Kill()).unwrap();
    }

    pub fn execute(&self, callback: Box<dyn FnOnce() + Send + 'static>) {
        self.sender.send(Message::Execute(callback)).unwrap();
    }

}
