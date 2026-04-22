**Terminology**
    - *Thread Pool* - a group of spawned threads that are ready and waiting to handle a task. When a program has a task it needs to complete it asignes an available thread to it. 
    - *Worker* - a structure that picks up the code and runs it in its thread.

**Tips**
    - Approach to create a multithreaded server recommended by the Rust Book is to have a queue of requests with `N` threads available. But there are other ways, which I would love to implement as well:
        1. fork/join model
        2. the single-threaded async I/O model
        3. multithreaded async I/O model
    - Write the client facing interface first.