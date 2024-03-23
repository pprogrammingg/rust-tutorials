use std::{sync::Mutex, time::Duration, thread::sleep};

/** 
 * Imagine there are 1_000_000 bricks that will be used to make houses, 
 * incorporating concepts such as workers, worker's capacity (# of bricks = 20) 
 * to carry and max # of workers on a house build site (e.g. 5) to build a house.  
 * Each worker takes 2 min to reach the house building sites. When arrives there 
 * will be a queue where worker is assigned a job sites. Each house takes 400 bricks 
 * to be built, each worker takes 100 second to lay its bricks. Workers who are 
 * then make a 1 min trip back to brick site, where they stand in a queue to go 
 * inside. The brick site can hold maximum 3 workers and loading bricks takes 20 
 * seconds. Once worker has all the bricks it needs it heads over to the house 
 * building site queue.
 */

enum WorkerStates {
    LOADING_BRICKS,
    AT_BRICK_SITE_QUEUE,
    TRAVELLING_TO_BUILD_SITE,
    AT_BUILD_SITE_QUEUE,
    LAYING_BRICKS,
    TRAVELLING_TO_BRICK_SITE
}
   
struct Worker {
    id: u8,
    current_state: WorkerStates,
}

struct House {
    completed: bool,
    number_of_bricks: u16
}



pub fn build_hempcrete_community(){

    const BRICK_TO_BUILD_SITE_TIME: u8 = 120; // seconds
    const BUILD_TO_BRICK_SITE_TIME: u8= 60; // seconds
    const WORKER_BRICK_LAY_TIME: u8 = 100; // seconds
    const TOTAL_NUMBER_OF_WORKERS: u8 = 20; 

    let bricks_ramining = Mutex::new(1_000_000);
    
    // 

    let build_house = |worker: Worker| {
        loop {
            // stand in brick site queue, for now just sleep 100 milliseconds
            sleep(Duration::from_millis(100));

            // load bricks

            sleep(Duration::from_millis(100));
        }
    };


    for i in 1..=TOTAL_NUMBER_OF_WORKERS {
        println!("{i}");
    }




}