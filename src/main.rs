mod traits_tutorial;
mod multi_threaded_basic;
mod multi_threaded_scope;

// use multi_threaded_basic::multi_threaded_example;
use multi_threaded_scope::test_thread_variable;

fn main() {
    // Run the traits tutorial example
    //run_traits_example();

    // Run the multi-threaded Rust example
    // multi_threaded_example();

    test_thread_variable();
}