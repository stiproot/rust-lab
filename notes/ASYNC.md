
Each await point—that is, every place where the code uses the await keyword—represents a place where control is handed back to the runtime.

To make that work, Rust needs to keep track of the state involved in the async block so that the runtime can kick off some other work and then come back when it’s ready to try advancing the first one again.
This is an invisible state machine.

The Rust compiler creates and manages the state machine data structures for async code automatically.

Ultimately, something has to execute this state machine, and that something is a runtime.
(This is why you may come across references to executors when looking into runtimes: an executor is the part of a runtime responsible for executing the async code.)
