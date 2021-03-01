use std::thread;
use std::sync::Arc;
use crossbeam::atomic::AtomicCell;

fn main() {
    let total_flips: Arc<AtomicCell<u64>> = Arc::new(AtomicCell::new(0));
    let completed: Arc<AtomicCell<u64>> = Arc::new(AtomicCell::new(0));
    let runs = 8;
    let target_flips = 10;

    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            simulate(target_flips, total_flips);
            completed.fetch_add(1);
        });
    }

    while completed.load() < runs {}
    println!("Final average: {}", total_flips.load() / completed.load());
}

fn simulate(target_flips: u64, total_flips: Arc<AtomicCell<u64>>) {
    let mut consecutive = 0;
    let mut iterations = 0;
    while consecutive < target_flips {
        iterations += 1;
        if rand::random() {
            consecutive += 1;
        } else {
            consecutive = 0;
        }
    }
    println!("iterations: {}", iterations);
    total_flips.fetch_add(iterations);
}