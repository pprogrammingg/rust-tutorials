# Async/Await

- is equivalent to a regular function but with return value of type `Future<output=SOME_TYPE>`
- `await` is basically to wait for a piece of code to complete before proceeding to the next
- `select!` can be used in a loop to evaluate different branches of await and see whether progress can be made
    - Could be inefficient in that does not make use of unsued cycles, one branch being ready and the other not ready etc
- `join!` runs things more concurrently and more efficient than select for case we want all awaits to resolve
- Executors such as the one from `Tokio` essentially annotate the main as async to mean (main actually cannot become async). 
The annotation is sugar code for declaring an executor which runs  a parent async block.
- As such, eventhough with `join!` and `select!` some level concurrency is achieved, it is essentially done in one place.
Remember concurrency is task switching, parallelism is simultaenous tasks.
- To achieve paralellism spawn different threads to contain other async code. The spawned code can use an event distribution 
or tracing system to catch any errors.
- for expensive functions that need awaiting may wanna use `spawn_blocking`
- How are async code state variables stores?
    - Underlying the code is divided to chunks like pretty await state vars, the await line and then final chunk vars. A state
    machine is created with all varialbr from chunk1 and chunk2
    - as such any other process containing the future from another code, needs to copy around statemachine, as such `tokio::spwan` 
    cam be used to store a pointer to the statemachine ofr the OG 
-`async traits` since size of fututre is not known use `tokio async_trait` which causes return values to be heap allocated dynamically
dispatched variable (`Pin<Box<dyn<Future<output=_>>>`). The cons of this is memory allocation pressure and extra pointer indireciton.
As such `async_trait` is best for higher level functions not bottom of stack
- Tokio Mutex vs STD Mutex : Tokio Mutex is async aware, if another thread locks the mutex, the current thread needing it, will just 
yield (return until the other one is done). Unless the other thread is doing a trivial task and not having to wait a long time in
which case STD Mutex is fine.
- Regular threads vs Tokio Spawned Tasks: Tokio tasks are cooperatively scheduled (yield rather block) and given to executor whereas regular threads are entirely 
OS created threads that run in paraellel with no executor, as such OS can distrupt any time.
- Tracing and Async: if `foo()` awaits a future it means it puts it in the queue of executor. As such if the future panic, foo() is not mentioned in the tracing.
You can use `trace_future` crate to name the future (you can include the function name that spawned it)
- By defualt Tokio has no run-time, you have to bring it in as a flag (like `rt-multi-thread`)
- Terminology:
    - `indirection` when results of a line of code is not available immediate e.g. in Future, Promise, etc
    - `pointer indirection` accessing the value being pointed to 
