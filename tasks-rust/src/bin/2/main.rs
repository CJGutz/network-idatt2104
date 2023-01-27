use workers::Workers;

pub mod workers;

fn main() {
    let mut workers = Workers::new(2);
    workers.start();
    
    workers.post_timeout(|| {
        println!("Hello from the thread");
    }, 5000);
    
    workers.join();

}
