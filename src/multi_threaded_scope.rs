pub fn test_thread_variable(){
    let age = 42;

    let my_closure = move || {
        println!("My parent variable is {age}");
    };

    let _result = std::thread::spawn(my_closure).join();

    println!("finished main!")
}