mod traits_tutorial;
mod multi_threaded_basic;
mod multi_threaded_scope;
mod multi_threaded_mutex;

use multi_threaded_mutex::mutex_exmaple;
// use multi_threaded_basic::multi_threaded_example;
//use multi_threaded_scope::{test_thread_variable_simple, test_thread_variable_with_scope};

fn main() {
    // Run the traits tutorial example
    //run_traits_example();

    // Run the multi-threaded Rust example
    // multi_threaded_example();

   // test_thread_variable_with_scope();

   mutex_exmaple();
}