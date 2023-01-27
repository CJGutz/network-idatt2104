use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

pub struct Workers {
    number_of_workers: u32,
    tasks: Arc<Mutex<Vec<fn()>>>,
    threads: Vec<thread::JoinHandle<()>>,
    condvar: Arc<(Mutex<bool>, Condvar)>,
}

impl Workers {
    pub fn post(&mut self, func: fn()) {
        self.tasks
            .lock()
            .expect("Could not lock task mutex")
            .push(func);

        let (lock, condvar) = &*self.condvar;
        let mut task_created = lock.lock().expect("Could not lock condvar mutex");
        *task_created = true;
        condvar.notify_one();
    }

    pub fn join(self) {
        self.threads.into_iter().for_each(|thread| {
            thread.join().expect("Could not join thread");
        });
    }

    // Create a new workers instance
    pub fn new(workers: u32) -> Workers {
        return Workers {
            number_of_workers: workers,
            tasks: Arc::new(Mutex::new(Vec::new())),
            threads: Vec::new(),
            condvar: Arc::new((Mutex::new(false), Condvar::new())),
        };
    }

    // Threads will wait for a task to run
    pub fn start(&mut self) {
        for _ in 0..self.number_of_workers {
            let (lock, condvar) = &*self.condvar;
            let mut task_created = lock.clone().lock().expect("Could not lock condvar mutex");

            let tasks = self.tasks.clone();
            self.threads.push(thread::spawn(move || loop {
                let task = tasks
                    .lock()
                    .expect("Could not lock task mutex when checking for tasks")
                    .pop();
                match task {
                    Some(task) => task(),
                    None => continue,
                }
            }));
        }
    }

    pub fn post_timeout(&mut self, func: fn(), timeout: u64) {
        thread::sleep(Duration::from_millis(timeout));
        self.tasks
            .lock()
            .expect("Could not lock task mutex at post with timeout")
            .push(func);
    }
}
