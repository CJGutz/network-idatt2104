use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

pub struct Workers {
    number_of_workers: u32,
    tasks: Arc<Mutex<VecDeque<fn()>>>,
    threads: Vec<thread::JoinHandle<()>>,
    condvar: Arc<(Mutex<bool>, Condvar)>,
    active: Arc<Mutex<bool>>,
}

impl Workers {
    pub fn post(&mut self, func: fn()) {
        self.tasks
            .lock()
            .expect("Could not lock task mutex")
            .push_back(func);

        let (lock, condvar) = &*self.condvar;
        let mut task_present = lock.lock().expect("Could not lock condvar mutex");
        *task_present = true;
        condvar.notify_one();
    }

    pub fn post_timeout(&mut self, func: fn(), timeout: u64) {
        thread::sleep(Duration::from_millis(timeout));
        self.post(func);
    }

    // Create a new workers instance
    pub fn new(workers: u32) -> Workers {
        return Workers {
            number_of_workers: workers,
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            threads: Vec::new(),
            condvar: Arc::new((Mutex::new(false), Condvar::new())),
            active: Arc::new(Mutex::new(true)),
        };
    }

    // Threads will wait for a task to run
    pub fn start(&mut self) {
        for _ in 0..self.number_of_workers {
            let active = self.active.clone();
            let tasks = self.tasks.clone();
            let convar_c = (&self.condvar).clone();
            self.threads.push(thread::spawn(move || {
                while *active.lock().expect("Could not lock active mutex") {
                    // Wait for a task to be posted
                    let (lock, condvar) = &*convar_c;
                    {
                        let mut task_present =
                            lock.lock().expect("Could not lock mutex for condvar");
                        while !*task_present {
                            task_present =
                                condvar.wait(task_present).expect("Failed to wait for task")
                        }
                    }

                    // Run task if possible
                    let task = tasks
                        .lock()
                        .expect("Could not lock task mutex when checking for tasks")
                        .pop_front();
                    match task {
                        Some(task) => task(),
                        None => (),
                    }

                    // Set no task present if task list empty
                    let (lock, _condvar) = &*convar_c;
                    let mut task_present = lock.lock().expect("Could not lock mutex for condvar");
                    *task_present = !tasks.lock().expect("Could not lock task mutex").is_empty();
                }
            }));
        }
    }

    pub fn join(self) {
        self.threads.into_iter().for_each(|thread| {
            thread.join().expect("Could not join thread");
        });
    }

    pub fn stop(self) {
        *self.active.lock().expect("Could not lock active mutex") = false;

        let (lock, condvar) = &*self.condvar;
        let mut task_present = lock.lock().expect("Could not lock mutex for condvar");
        *task_present = true;
        condvar.notify_all();
    }
}
