# observer-pattern
Observer pattern, implemented in Rust.

This crate contains an interface for Subject object, which uses function pointers (callbacks) to notify its observers. Function pointers allow the observers to be more loosely coupled and reduces branching, which reduces indirections and adds to performance.
This interface also allow the subject to apply any change using function pointers. This is to make the change state function generic giving more flexibility.

A drawback of this architecture is that the observer now does not necessarily have a state of its own, i.e. we do not enforce the observer to have its own state. However, its now a user choice to opt for a stateful observer by constructing closures that update observer's state.

Just some thoughts for the future:
Currently, this is a binary but I would rather have this as a library. Even cooler transformation would be to a proc macro library, but that will be highly opinionated. Some real world use case would help shape the future of this repo as a proc macro lib.
