// Example demonstrating multi-threaded programming in Rust

use std::{thread, time::Instant};

// Function that says hello in a separate thread
pub fn no_thread_example() {
    let mut x: u128 = 0;

    for i in 0..50 {
        x += i;
        println!("\x1b[38;2;100;100;25mvalue of i  is : {i}\x1b[0m");
    }
    println!("\x1b[38;2;100;100;25mMain thread finished a little bit early!\x1b[0m");
}

pub fn multi_threaded_example() {
    let thread_fn = || {
        let mut x: u128 = 0;

        for i in 0..500 {
            x += i;
            println!("value of i  is : {i}");
        }
        println!("value of x is : {x}");
    };

    let start_time = Instant::now();

    println!("Starting new worker thread!");

    // Spawn threads
    let handle = thread::spawn(thread_fn);
    let handle2 = thread::spawn(thread_fn);

    loop {
        no_thread_example();
        if handle.is_finished() && handle2.is_finished() {
            println!("Both workers completed!");
            break;
        }
    }

    let end_time = Instant::now();

    let duration = end_time.duration_since(start_time);
    let nanoseconds = duration.as_nanos();

    println!("Duration in nanoseconds: {}", nanoseconds);
}
