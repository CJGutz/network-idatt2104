use workers::Workers;

mod workers;

fn main() {
    let mut workers = Workers::new(2);
    workers.start();

    workers.post(|| {
        println!("Hello 1 from the thread");
    });

    workers.join();


}
