use std::env;

use crate::minigrep::minigrap;

mod guessing_game;
mod hempcrete_community;
mod http_reqwest_example;
mod minigrep;
mod mpsc_example;
mod multi_threaded_basic;
mod multi_threaded_mutex;
mod multi_threaded_scope;
mod traits_tutorial;

// use guessing_game::guessing_game;
// use hempcrete_community::build_hempcrete_community;
// use mpsc_example::send_from_main_receive_in_thread;
// use multi_threaded_mutex::mutex_exmaple;
// use multi_threaded_basic::multi_threaded_example;
// use multi_threaded_scope::{test_thread_variable_simple, test_thread_variable_with_scope};
// use http_reqwest_example::reqwest_example;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    ();

    // Run the multi-threaded Rust example

    // multi_threaded_example();

    // test_thread_variable_with_scope();

    // mutex_exmaple();

    // send_from_main_receive_in_thread();

    //  build_hempcrete_community();

    // reqwest_example();

    // guessing_game();

    // run_traits_example

    //-----
    // minigrep
    let (query, filepath) = minigrep::parse_config(&args);

    minigrep::minigrap(query, filepath);
}
