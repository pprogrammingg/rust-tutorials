# Gotchas of RUST

## Numbers
`let x = 1_000.000_1` is f64 by default not f32. To get f32 need to do this `let x: f32 = 1_000.000_1f32` (explicit declaration)
explicitely specifying for f32 (resulting in taking half as much space than f64) because a performance boost of large arrays of 
such numbers, as for individual values, compiler makes the necessary optimizations

`assert!(0.1+0.2==0.3);` // default f64 is too precise for example result could be 0.30000000000001, explicitely making each number 
post-fixed with f32 (e.g. 0.3f32) or 
It's generally recommended to avoid direct equality comparisons (==) between floating-point numbers due 
to precision issues. Instead, use comparisons with a small tolerance (e.g., approx_eq! or similar macros 
from crates like approx or implement custom comparisons with a threshold) to account for potential
rounding errors inherent in floating-point arithmetic.

# Flow
use `panic!()` or `unimplemented!` pr `todo!()` to do divergant functions

# Stack Memory
Variables pushed on stacks faster than heap allocation due to fixed sized (float, int, bool, etc.)
Type of unknown size will go on heap, and pointer of usize is returned
`String` type if mutable, it is stored on stack as 3 parts : 
`pointer`, `len` and `capacity` (size is # of stack cells * size per cell e.g. 3 x 8 bytes = 24 byts)
actual value of string will be on the heap.

## Copy vs Move
fixed size values which go on Stack can be copied but copying heap is expensive, instead Rust moves those.

# Ownership
In Rust, ownership rules govern how memory is managed and accessed, ensuring memory safety without the need for a garbage collector. The key ownership rules are:

1. Each value in Rust has a single owner: Every piece of data has a variable that is its owner. When the owner goes out of scope, the value is dropped.

2. Ownership is exclusive: There can only be one mutable reference (&mut) to a piece of data in a particular scope, preventing data races and concurrent mutation.

3. Borrowing and references: Multiple immutable references (&) to a piece of data are allowed, but they cannot coexist with a mutable reference. References must also follow the borrowing rules: a reference cannot outlive the data it refers to.

The borrowing rules prevent data races: Rust's ownership system guarantees memory safety at compile time, ensuring that references are always valid and that there are no dangling pointers or references to freed memory.

Move semantics: When a value is assigned to another variable or passed to a function, ownership of that value is moved. This prevents accidental use of invalidated or deallocated memory.

These rules help prevent common memory-related bugs like null pointer dereferencing, dangling pointers, or data races. They're enforced by Rust's compiler, which checks ownership and borrowing at compile tim

`s.into_bytes()` consumes `s`, thereby if still need to use it do `s.clone().into_bytes()`
or use `s.as_bytes()` which uses `&self` not `self` as argument
`let s1 = String::from("some_str")` and `let s2 = s1`, pointer gets copied, but second rule of ownership
implies that there should be only 1 owner. `s1` is dropped then
partial move: in destructuring we can use `ref` to make receiving variable get the field as reference. 
e.g. ` let Person { name, ref age } = person;`

# Borrowing 
- A way to view or mutate data without taking ownership
- Borrowing takes the pointer not the data itself (prevent dangling pointers and data races)
- Can only have one mutable reference to a memory location OR any number of immutable references at the same time

```rust
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s; --> Problem, having a mutable reference along with immutable ones
```
- References should always be valid

```rust
    let ref_to_nothing = get_dangling_ref();
    
    def get_dangling_ref() -> &String {
        let s = String::from("hello");
        &s
    }

``` 

- Example where an immutable reference is still in-use when `s.clear()` which take `&mut self` as argument is invoked. 

```rust
    fn main() {
        let mut s = String::from("hello world");

        // Here, &s is `&String` type, but `first_word` need a `&str` type.
        // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
        let word = first_word(&s);
        

        s.clear(); // error! --> move this line to the bottom to make things work
        println!("the first word is: {}", word); 
    }

    fn first_word(s: &str) -> &str {
        &s[..1]
    }
```

# A Note about cloning vs using references
- Clone means full copy and as such more time and memory is used; however, it migth be more straight forward than using refrences and not to have to worry about managing lifetimes.

# Compound Type
## str, &str and String
- str: string slice representing a valid sequence of UTF-8 bytes. Its size known at compile time.
- &str: a reference to string slice and is immutable and fixed-size.
- String: is mutable, owned and heap-allocated, not-null terminated, stored as a vector of valid UTF bytes
- Are string literals stored in stack? No, stored in program binary inside static memory where all the constants, static vars and static literals are stored.
- deRef Coersion: allows Rust to automatically coerce types when using references in certain situations, including function arguments. When you pass an owned String to a function expecting a &str, Rust performs deref coercion, which automatically converts the String to a &str reference if needed.
- print address of a `String` via 

```rust
    let raw_ptr = s.as_ptr(); // Get a raw pointer to the buffer
    println!("{:?}", raw_ptr);
```
- String index: cannot access a char via index in String but can use a slice &s1[start..end]
- byte string are array of bytes that do not have to be UTF-8. Can escape byte sequence but not unicode
- Slicing a string
```rust
   let s1 = String::from("hi,中国");
   let h = &s1[0..1]; // letter h takes 1 byte
   let utf8_char = &s1[3..6] // print 中 as it takes 3 bytes
```
- iterationg chars with `chars()` method
```rust
fn main() {
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
```

## Arrays
- use `get` instead of unsafe index to get an element
```rust
    let names = [String::from("Sunfei"), "Sunface".to_string()];
        
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();
```

## Slices
- Here, both [i32] and str are slice types, but directly using it will cause errors. You have to use the reference of the slice instead: &[i32], &str.

```rust
    // Fix the errors, DON'T add new lines!
    fn main() {
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..2];

        let s2: &str = "hello, world" as &str;

        println!("Success!");
    }
```
- Slice reference (or slice for simplicity) is a two-word object. First word is reference to data, second is length. so any N element slice (number of elements don't matter) are ` assert!(std::mem::size_of_val(&slice) == 2 * 8);` if each word size is 8 bytes then 2 words gives 16.

## Struct
- Unit struct useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
- tuple structs: when you want to name a struct but do not care about the field names

## Enum
- Option Enum
```rust
    fn main() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        if let Some(n) = six {
            println!("{}", n);

            return;
        } 
            
        panic!("NEVER LET THIS RUN！");
    } 

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
```

# Flow
## Iteration
- in the example array of String (owned objects) needs to be access by reference if it is to be used after the loop, but the same does apply to array of primitives
- Also note we do not have to derefence an element read from array ref for `println!` dereferences automatically

```Rust
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names { // need to have &names as there is no copy with array of Strings
        println!("{}", name); // no need to dereference here, println! takes care
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        println!("{}", n);
    }
    
    println!("{:?}", numbers);
} 
```

- `loop {}` denotes infinite loop
- in assignment can use `break expression` to return a value to a variable
- It's possible to break or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.

## Match
- `if let` can be used to unwrap value of option enum
- Shadowing gotcha

```rust
fn main() {
    let age = Some(30);
    if let Some(mut age) = age { // Create a new variable with the same name as previous `age`
       assert_eq!(age, 30);
       age = 40; // --> why the 40 did not take effect ???
    } // The new variable `age` goes out of scope here
    
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }

```

## Pattern
- use `@` to extract a matched variable

```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id@ (3..=7),
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@ (10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```
- match gaurd example
``` rust
// Fill in the blank to make the code work, `split` MUST be used
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}
```

# Methods and Associated functions
- Methods operate on `self` (always first parameter) in the context of `struct`, `trait object` or `enum`. Note: The `&self` is actually short for `self: &Self`
- Associated functions are similar but they do not operate on `self`. Usually used for constructors.
    - Constructor made as associated function can return `Self`
 
# Generics
- Zero cost abstraction, flexible code 

```rust
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
```

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {

    // Try using `&self ` as argument does not work, gives error: 
    //  x: self.x,
    //          |^^^^^^ move occurs because `self.x` has type `T`, which does not implement the `Copy` trait
    fn mixup<V, W>(self, other_point: Point<V,W>) -> Point<T,W>{
        Point{
            x: self.x,
            y: other_point.y
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}
```

## Const Generics
- use generics with types whose size in known at compile time, for array sizes and bit widths, etc.

## Traits
- Similar to interfaces in some programming languages, a trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
- `Derive` macro to bring some automatic implemenations of traits
- Operator overload: the + operator in a + b calls the add method (as in a.add(b))
- Trait can be passed as function param , e.g. take any type that implements a certain trait

```rust
    fn myFunc(x: &impl some_trait){
        
    }
```
- Trait bounds are declared like genereic and put bound on generic types
```rust

    // function args can be different types but should implement the same trait
    fn myFunc(x: &impl some_trait, y: &impl some_trait){

    }

    // function args should be the same type AND implement the same trait
    fn myFunc<T: some_trait>(x: &T, y: &T) -> i32{}

    // can combine trait bounds
    fn myFunc<T: Clone, U: Clone + Debug>(t:&T, u:&U) {}

    //  `where` clause can also be use
    fn myFunc<T, U>(t:&T, u:&U) -> i32 where T: Clone, U: Clone + Debug {}

    // trait bound for return type
    fn myFunc() -> impl Animal {}

    // use lifetime and traitbounds
    fn greet_longer<'a, T: Greet + 'a>(x: &'a T) {
      println!("{}", x.greet());
    }

    // dynamic dispatch
    fn greet_dyn(x: &dyn Greet) {
      println!("{}", x.greet());
    }

    let person = Person;
    greet_dyn(&person)
```
- Note: use Trait Objects instead when you really need to return several types. Trait objects are essentially pointers to any type that implements that trait, whose precise type is only know at run-time. Syntax `Box<dyn Animal>` , why `Box` why not `&dyn Animal`

## Trait Bounds
- The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound.
When working with generics, the type parameters often must use traits as bounds to stipulate what functionality a type implements.

# Closures
- One difference with named functions is that closures can capture value from the parent scope
- Self is passed to closures automatically   
- Closures can take params by reference, mutable reference or just take ownership
- Example of closure using mutable ref
```rust
    fn main() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);
        // Cannot borrow the value again, as it is mutably referenced by closure. 
        // For example, we cannot use it for println
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }
```
- `move` keyword to give ownership is barely used. Thread is an example where a variable is moved to a thread from the context of current thread.
- How closure body uses the vars it captures, it will dictate which Fn traits it implements.
- in Option `unwrap_or_else` defintition, using `FnOnce` in the trait bound expresses the constraint that unwrap_or_else is only going to call f at most one time.

# Constants vs Static
- Constants: immutable and value must be known at compile time, their access depends on scope.
- Static: can be mutable if declared with `static mut` but the modification is done via unsafe code. Are lazily initialized (upon first time access). Can be initialized at run-time using static blocks. Are available to be access globally.

# Misc.
- `to_owned()`: `to_owned()` is a method in Rust that's used to create an owned value (allocated on the heap) from a borrowed value. It's commonly used to convert borrowed data types like &str or &[T] (slices) into owned data types like String or Vec<T>. `to_owned()` is essentially a shorthand for .clone() on types that implement Clone. 
Benefits of ownership:
1. Ownership and memory management: Rust's ownership system ensures memory safety. Converting borrowed types to owned types helps manage memory explicitly, as ownership rules differ between borrowed and owned data.

2. Avoiding lifetime issues: Using to_owned() can help avoid borrowing-related lifetime issues. Borrowed references (&) have lifetimes tied to their scope, but owned types like String or Vec<T> are not bound by the same lifetimes.

3. Semantic clarity: Converting between borrowed and owned types explicitly communicates ownership and intent within your code.

Noste: that the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value
- if not specified an integer by default is of type `i32`


# Multi-Threaded

- `cargo run` creates a `process` which can spawn `1 or more threads`. Each thread can run on `1 or more CPU sockets`, each `CPU socket can have 1 or more cores`. Through means like `hyperthreading` could give operating system impression that there are 16 cores whereby there are actually 8 for example.
- Scheduling of a thread on a CPU core is the job of a system / hardware optimizer
- Could have ways to specify which cores can our Rust program run on e.g. Linux programs like `nice`
- `std Library` has the `thread` module to look at, a relatively simple module: [thread](https://doc.rust-lang.org/std/thread/index.html)
- `thread` module has `spawn` function which receives a `closure` and returns a `JoinHandle<T>`
- Process/thread scheduling: some comments online are that MacOS handles it and in most cases we do not need to bother

## Scopeed Threads
- In Rust, scopes play a crucial role in managing resources and ensuring safety, especially in the context of multithreading. Rust's ownership and borrowing system helps prevent data races and other concurrency-related issues.

- Scopes in Rust are used in conjunction with ownership and borrowing rules to control the lifetime and accessibility of data. In the context of multithreading, Rust provides the std::thread module for creating and managing threads. When working with threads, scopes help define the lifetime of borrows and ownership, ensuring that references to data are valid throughout the thread's execution. This is essential for preventing data races and other concurrency bugs. 

- Scope gaurantees all threads will be joined automatically
- Scope also has explicite ways to join and get a handle

## Mutex

- `std::sync:mutex` provides a way to lock data by a thread for exclusive use
- lock to access/mutate data
- unlocks implicitely after scope of thread is finished
- if a thread locks Mutex but panics, other threads won't be able to unlock it unless Mutex is dropped.
- able to use a loop and try_unlock on Mutex, if lock obtained do work, or sleep if nothing happened
- play around with joining (waiting for thread to finish) or not joining and see what happens


# MPSC
- Multiple Producers Singel Consumer is a message passing technique to aid multi-threaded apps
- could be used in a single-threaded app, generally speaking in multi-threaded
- if drop sender or receiver or both channel just closes, cannot send to it anymore
- `.recv` is a blocking op, use `.recv_timeout` to be non-blocking for a certain amount of time


# ForkJoin, Pipeline and worker patterns
- fork join: `par_iter` of Rayon crate for parallel iteration
```rust
    use rayon::prelude::*;

    fn main() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let sum: i32 = data.par_iter().map(|&x| x).sum();

        println!("Sum: {}", sum);
    }
``` 
- worker pattern: pretty much thread spawning

# Sending Requests
- Crates like `reqwest` (blocking is an optional cargo, need to install it explicitely), `rust-curl` (binding to c-curl), `hyper`, `ureq` (just blocking IO no async - so fairly light)
- `mockoon` for mocking

# Cargo
- `cargo check` is faster than `cargo build` useful when you want to see whether code can compile
- `cargo build --release` is a lengthier process that does some optimizations. When benchmarking code use the release verison

# Env vars example
- see `minigrep.rs` for how env var is brought in
- example usage: 
`IGNORE_CASE=1 cargo run -- to poem.txt`
- `Remove-Item Env:IGNORE_CASE` to remove env var


# Iterators for more functional style of programming
- Chapter 13.3 of Rust book states:
`We can write this code in a more concise way using iterator adaptor methods. Doing so also lets us avoid having a mutable intermediate results vector. The functional programming style prefers to minimize the amount of mutable state to make code clearer. Removing the mutable state might enable a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the results vector`

# Cell, RefCell, Atomic Ref Cell and Inner Mutability
- Watched parts of [Jon Gjengset](https://www.youtube.com/watch?v=8O0Nt9qY_vo)
- Cell:
    - Cell -> impl with UnsafeCell type to enforce non-sharable between threads and something that does not give away pointer to the inner value.
    - Cells have the benefit of allowing them to be shared between multiple owners and provide ways to mutate the inner value (in a single thread)
- RefCell:
    - Dynamic check borrowing - great usecase for graphs and trees
    - keeps track of references
    - Can impl burrow and burrow_mut by stating that if there are no exclusive references, then can be burrowed immutably and only way to be immutably burrowed is to have exclusive reference (ref count = 1)
- RC:
    - Similar tp Refcell but does not allow mutability

# Rust Async/Await Overview

## Rust Async Fundamentals
- **`async` and `await`:**
  - `async` transforms a function into one that returns a `Future<Output = T>`, where `T` is the return type.
  - `await` pauses execution until the `Future` is ready (resolved or completed), but **doesn't block** the thread. It allows other tasks to run while waiting.

## Futures in Rust
- A **`Future`** represents a computation that hasn’t finished yet.
- When calling an `async fn`, it returns a **`Future<Output = T>`**.
- Futures are **lazy**, meaning they don't do anything until they are polled (using `await` or manually polling them).

## Pinning and Polling
- The async runtime polls futures to make progress.
- **Pinning** ensures that the future remains in memory at a fixed location because async functions can be suspended and resumed at different times.

## Concurrency with `select!`
- **`select!`** waits for **multiple futures** at once, allowing you to respond to whichever future completes first.
- Only **one** future will complete at a time; it’s useful when you have several tasks that can make progress independently, but you don’t need all to finish.
- Useful for multiplexing I/O or handling different events (like WebSocket messages or timer events).

## Parallelism with `join!`
- **`join!`** runs multiple futures **concurrently**, but waits for **all** of them to complete.
- All futures are polled concurrently, making it ideal for situations where you want tasks to run in parallel but require all results before proceeding.
- Example: Fetching data from multiple sources and using the results together once all have arrived.

---

## `select!` vs `join!`

- **`select!`:**
  - Only one future completes at a time; useful when you need to **react** to events or messages in real-time.
  - The remaining futures are canceled when the first one completes.
  - Comes from the **`futures`** crate and is often used in **Tokio** or other async runtimes, but isn’t part of the standard library.

- **`join!`:**
  - All futures run concurrently, and the program waits until all of them complete.
  - All futures continue to run, regardless of which finishes first.
  - Comes from the **`futures`** crate or async runtimes like **Tokio**.

---

## Spawning Async Tasks

- **`tokio::spawn`:**
  - Spawns a lightweight **task** that runs on an async runtime (like Tokio).
  - Tasks are **cooperatively scheduled**, meaning they yield control to the runtime when waiting for I/O or other async operations.
  - This enables **concurrent** execution, where multiple tasks share the same thread and make progress by switching between them.
  - Tasks don’t run in parallel, but **concurrently** on the same thread or across multiple threads (depending on the runtime configuration).

- **`thread::spawn`:**
  - Spawns a new **OS thread** for the task.
  - Threads run in **parallel**, each with its own stack, and the operating system handles preemptive scheduling.
  - OS threads are heavier than async tasks because they require more resources (memory for stacks) and are less efficient when running large numbers of tasks.

---

## Parallelism vs Concurrency

- **Concurrency:**
  - Tasks are scheduled cooperatively by the runtime. They take turns running, yielding when waiting for something (like I/O).
  - **`tokio::spawn`** is an example of concurrency. Multiple tasks share the same thread, and the runtime switches between them without running them simultaneously.
  - **Cooperative scheduling** ensures that tasks run only when they're ready, leading to more efficient I/O-bound tasks.

- **Parallelism:**
  - Tasks run **simultaneously** on separate threads or processors.
  - **`thread::spawn`** enables parallelism, as it creates a separate OS thread for each task.
  - Suitable for CPU-bound operations where multiple tasks can use separate cores for true parallel execution.

---

## Task Scheduling

- **`tokio::spawn` (async tasks)**:
  - **Cooperatively scheduled**: Tasks yield control when idle (e.g., waiting for I/O), which allows for concurrency. Multiple tasks can make progress without using multiple OS threads.
  - **Concurrency**, not true parallelism unless explicitly configured to use multiple worker threads.

- **`thread::spawn` (OS threads)**:
  - **Preemptively scheduled**: The OS can interrupt a thread and switch between them, achieving true parallel execution.
  - **Parallelism**, as tasks run on different threads at the same time.

---

## Runtime Differences

- **Standard Library (`std`) vs `Tokio`:**
  - The **`std::thread::spawn`** and **`std::future::Future`** are part of Rust’s standard library.
  - However, **`async/await`, `select!`, `join!`, and `tokio::spawn`** are not part of the standard library. They require an async runtime like **Tokio**, **async-std**, or the **`futures`** crate to function.
  - These runtimes provide additional functionality, like task scheduling, I/O handling, timers, and utilities for running async code efficiently.

---

## `tokio::main` Macro

- The **`tokio::main`** macro is a convenient attribute that sets up an async context inside the `main` function.
  - When you mark a function with `#[tokio::main]`, it effectively **wraps the `main` function** in a **Tokio runtime**.
  - It allows you to write an async `main` function (e.g., `async fn main() { ... }`), making it easy to run asynchronous code right from the entry point.
  - It initializes the async runtime and spawns the async tasks.
  - Depending on the **`flavor`** you choose (e.g., `multi_thread` or `current_thread`), it will configure how the runtime manages tasks and threads.

# Fat Pointers
In Rust, a fat pointer is a pointer that carries extra information beyond just the memory address. 
Fat pointers are used when the size or layout of the data being referenced isn't known at compile time. 
There are two common situations where Rust uses fat pointers:

## Trait Objects `&dyn trait`
When you use a trait object (e.g., &dyn SomeTrait), Rust doesn't know at compile time which 
specific type will be behind that trait. A fat pointer is used to store:

1. A pointer to the actual data (the concrete type that implements the trait). 
2. A pointer to a vtable (virtual method table) that holds information about the 
    methods defined in the trait for the concrete type.

## Slices (&[T])
Slices, like &[T], are another example of fat pointers. The size of a slice ([T]) is unknown at compile time, so a fat pointer is used to store:
A pointer to the start of the slice.
The length of the slice (i.e., the number of elements in it).

1. A pointer to the start of the slice.
2. The length of the slice (i.e., the number of elements in it).

### Why are they called "fat" pointers? and what are they used for?
They're called "fat" because they carry more than just the address of the data. A regular or "thin" pointer (like &T) only holds the memory address, while a fat pointer holds both a pointer to the data and additional 
information (e.g., vtable pointer for trait objects, length for slices).

Rust uses these fat pointers behind the scenes to enable dynamic behavior and handling of unsized types like trait objects and slices while 
ensuring performance and memory safety.


## Some Topics to Learn/Expand on
- How does Rust achieve memory safety?
- What is stack vs heap in Rust? What happens when a Vec object is declared? Syscalls what are they and talk about them in terms of efficient Rust program? What are atomic types?
- What are some popular datastructures in STD lib?
- Asynchronous Rust, tasks and thread (pros and cons)
- What are "fat pointers"? What is "VLookup" table?
- BTreeSets are great for converting a Vec to ordered and unique
- often working with Vec might get int to situtation where item type is a reference eventhough a primitive type
like `&Vec<&i32>`, this can be converted to non-referenced version by using `.copied` 

e.g in the below exmaple without `.copied` we end up with `Vec<&i32>`
```rust
fn unique_ordered_set(input: &Vec<i32>) -> Vec<i32> {
    
      // Convert Vec to BTreeSet to get unique, sorted numbers
      let unique_seq: BTreeSet<&i32> = input.into_iter().collect();

      // If needed, you can convert back to Vec
      let unique_sorted_vec: Vec<i32> = unique_seq.into_iter().copied().collect();


      unique_sorted_vec
}

```


<!-- Feb 2025 CheatSheet -->
# Stirng Manipulation
- for (i, char) in some_string.char_indecies() // i here is byte position, so unicode char can take more bytes, hence i is not linearly increasing
- for (i, char) in some_string.chars().enumerate() // i is char index strictly
- lower_case
    - to_ascii_lowercase() faster/works only with ascii, to_lowercase() works with unicode also, but need to use `next().unwrap()` to go through different lowercases of the same char as the result slower

# Vector
- for (i, e) in some_vector.iter().enumerate()
- Vec<char> to String 

```rust
    let list = Vec!::new(['a', 'b', 'c']);
    let list_stringified = list.iter().collect();
```

# Loops and Iterators

# Multi-Threading and Concurrency Summary
