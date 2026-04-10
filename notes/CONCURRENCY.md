
`Mutex` is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. 
To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.

The `lock` is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. 
Therefore, the mutex is described as guarding the data it holds via the locking system.