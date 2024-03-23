mod traits_tutorial;
mod multi_threaded_basic;
mod multi_threaded_scope;
mod multi_threaded_mutex;
mod mpsc_example;
mod hempcrete_community;
mod http_reqwest_example;


// use hempcrete_community::build_hempcrete_community;
// use mpsc_example::send_from_main_receive_in_thread;
// use multi_threaded_mutex::mutex_exmaple;
// use multi_threaded_basic::multi_threaded_example;
// use multi_threaded_scope::{test_thread_variable_simple, test_thread_variable_with_scope};
use http_reqwest_example::reqwest_example;
fn main() {
    // Run the traits tutorial example
    //run_traits_example();

    // Run the multi-threaded Rust example
    // multi_threaded_example();

   // test_thread_variable_with_scope();

   // mutex_exmaple();

   // send_from_main_receive_in_thread();

   //  build_hempcrete_community();

   reqwest_example();
}