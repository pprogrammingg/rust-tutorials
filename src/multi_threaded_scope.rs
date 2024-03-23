use std::thread::Scope;

struct Person { age: u32, name: String}

pub fn test_thread_variable_simple(){
    let age = 42;
    let person01 = Person {age, name: String::from("Trevor")};

    // notice move is necessary to give the closure access to the parent variable
    let my_closure = move || {
        println!("age is {age}");
        println!("name is {}", person01.name);
    };

    println!("My parent variable is {age}"); // runs OK
    // println!("name is {}", person01.name); // Error! person01 is allocated on the heap and not simply copied 
    // to the child closure like age. 
    let _result = std::thread::spawn(my_closure).join();
    println!("My parent variable is {age}"); // runs OK

    println!("finished main!")
}

pub fn test_thread_variable_with_scope(){
    let age = 42;
    let person01 = Person {age, name: String::from("Trevor")};

    // notice move is necessary to give the closure access to the parent variable
    let my_closure =  || {
        println!("Closure vars:");
        println!("age is {age}");
        println!("name is {}", &person01.name); // &person01 is not necessary?
    };

    println!("age is {age}"); // runs OK
    println!("name is {}", person01.name); 


    std::thread::scope(|scope: &Scope<'_, '_>| {
        scope.spawn(my_closure);
    });

    
    println!("Thread is done");
    println!("age is {age}");// runs OK
    println!("name is {}", person01.name);
    println!("finished main!")
}