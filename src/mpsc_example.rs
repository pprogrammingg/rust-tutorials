use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

/*
* 1. declare a channel
* 2. define an ananymous function that in a loop listens to channel and prints message if receive success
* 3. If failed to receive, inrement a failure count until 10 reached, upon which break the loop
* 4. Use main thread to send bunch of messages, sleep a bit in beteen
* 5. Use thread to fire the process msg
*/
pub fn send_from_main_receive_in_thread() {
    // 1.
    let (tx, rx) = mpsc::channel::<String>();

    let process_msg = move || {
        let mut failure_count = 0u8;

        loop {
            println!("Attempting to receive message!");
            // 1.
            let result = rx.recv_timeout(Duration::from_millis(300));

            if result.is_ok() {
                failure_count = 0;
                println!("result {}", result.unwrap());
            } else {
                // 3.
                println!("Failed to receive!");
                failure_count += 1;
            }

            // 3.
            if failure_count > 10 {
                println!("Stop listening!");
                break;
            }
            // sleep(Duration::from_millis(100));
        }
    };

    let worker = thread::spawn(process_msg);

    // 4.
    for i in 0..=10 {
        let result = tx.send(i.to_string());
        println!("Send result is {}", result.is_ok());
        sleep(Duration::from_millis(1000));
    }

    let _result = worker.join();
    println!("Exiting program!");
}
