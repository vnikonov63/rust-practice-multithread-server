``cargo run -- help`` to see all of the possible commands, supported by the application

``cargo run -- spawn-infinite-threads`` to spawn a separate thread for every incoming request to the server

``cargo run -- thread-pool <num>`` to have a pool of `num` threads where each of the requests is processed separately

### Terminology
- **Thread Pool** - a group of spawned threads that are ready and waiting to handle a task. When a program has a task it needs to complete it asignes an available thread to it. 
- **Worker** - a structure that picks up the code and runs it in its thread.

### Learning outcomes from this project
- Write the client facing interface first.