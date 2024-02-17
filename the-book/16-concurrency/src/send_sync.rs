// * Allowing Transference of Ownership Between Threads with `Send` *
//
//      The `Send` marker trait indicates that ownership of values of 
//      the type implementing `Send` can be transferred between threads.
//
//
//
// * Allowing Access from Multiple Threads with `Sync` *
//
//      The `Sync` marker trait indicates that it is safe for the type 
//      implementing `Sync` to be referenced from multiple threads. 
//      In other words, any type T is Sync if &T (an immutable reference to T) is `Send`, 
//      meaning the reference can be sent safely to another thread.
//
//
//
// * Implementing `Send` and `Sync` Manually Is Unsafe *
//
//      Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and `Sync`, 
//      we don’t have to implement those traits manually. 
//      As marker traits, they don’t even have any methods to implement. 
//      They’re just useful for enforcing invariants related to concurrency.