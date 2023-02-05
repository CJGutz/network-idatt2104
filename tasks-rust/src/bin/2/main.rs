use std::thread;

use tasks_rust::workers::Workers;

fn print_hello() {
    println!("Hello from thread {:?}", thread::current().id());
}


fn main() {
    let mut workers = Workers::new(4);
    let mut event_loop = Workers::new(1);
    workers.start();
    event_loop.start();

    workers.post(print_hello);

    workers.post_timeout(print_hello, 10);

    event_loop.post(|| {
        println!("Hello from event loop");
    });

    workers.post(print_hello);

    workers.join();
}
