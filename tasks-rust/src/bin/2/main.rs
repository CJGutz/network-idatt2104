use std::{thread, time::Duration};

use workers::Workers;

mod workers;

fn main() {
    let mut workers = Workers::new(4);
    let mut event_loop = Workers::new(1);
    workers.start();
    event_loop.start();

    workers.post(|| {
        println!("Hello 1");
    });

    workers.post_timeout(|| println!("Hello 2"), 10);

    event_loop.post(|| {
        println!("Hello from event loop");
    });

    workers.post(|| {
        println!("Hello 3");
    });

    workers.join();
}
