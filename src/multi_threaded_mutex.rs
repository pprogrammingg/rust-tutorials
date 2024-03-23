use std::thread::{scope, sleep};
use std::time::Duration;
use std::{ops::AddAssign, sync::Mutex};

pub fn mutex_exmaple() {
    let score = Mutex::new(0u16);

    let my_func = || {
        println!("\x1b[38;5;190m Thread 1 waiting to lock score mutex!\x1b[0m");
        let mut data = score.lock().unwrap();
        println!("Thread 1 obtained lock!");
        for i in 1..10 {
            data.add_assign(i);
            println!("\x1b[38;5;190m adding {i}!\x1b[0m");
            sleep(Duration::from_millis(400));
        }

        println!("\x1b[38;5;190m Thread 1 finished its work!\x1b[0m");
    };

    let my_func2 = || {
        println!("Thread 2 waiting to lock score mutex!");
        // usually no need to drop as exiting scope naturally drops the lock on mutex.
        // unless there is a panic! or something that forces us to release the mutex

        // drop(data);
        // panic!("thread 2 paniced!!!");

        loop {
            println!("Thread 2 wait a bit -  try to obtain lock later");
            sleep(Duration::from_millis(200));
            let gaurd = score.try_lock();
            if gaurd.is_ok() {
                println!("Thread 2 obtained lock!");
                let mut data = gaurd.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("adding {i}!")
                }
                println!("Thread 2 finished its work!");
                break;
            }
        }
    };

    scope(|scope| {
        // let handle1 = scope.spawn(my_func).join();
        // let handle2 = scope.spawn(my_func2).join();
        scope.spawn(my_func);
        scope.spawn(my_func2); // thanks to scoped thread with automatically join
                               // if handle2.is_err(){ // more graceful exit
                               //         println!("Handle thread 2 error here!");
                               // }
    });

    println!("score is {:?}", score.lock().unwrap());
}
