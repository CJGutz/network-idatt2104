use std::time::Instant;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        println!("Please provide three arguments: start, stop, number of threads");
        return;
    }

    let start = args[1].parse::<u32>().unwrap();
    let stop = args[2].parse::<u32>().unwrap();
    let number_of_threads = args[3].parse::<u32>().unwrap();

    let primes: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    let time_start = Instant::now();

    for i in 0..number_of_threads {
        let prime_copy = primes.clone();
        threads.push(thread::spawn(move || {
            for j in ((start + i)..stop).step_by(number_of_threads as usize) {
                if is_prime(j) {
                    prime_copy.lock().unwrap().push(j);
                }
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let time_end = Instant::now();

    for prime in primes.lock().unwrap().iter() {
        print!("{}, ", prime);
    }
    println!("\nFound {:?} prime\n", primes.lock().unwrap().len());
    println!(
        "Used {:?} ms",
        time_end.duration_since(time_start).as_millis()
    );
}

fn is_prime(num: u32) -> bool {
    if num == 2 {
        return true;
    }
    if num % 2 == 0 || num <= 1 {
        return false;
    }
    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
