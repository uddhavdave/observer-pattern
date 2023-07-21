# observer-pattern
Non conventional observer pattern, implemented in Rust

Non conventional, in the sense that it is opinionated. 
Some extra methods in the interface:
- enable the use of observers after they have registerd to the subject (Expected). To achieve this in Rust, we have to use Runtime borrow checking concepts, which makes the trait methods complex. Hence some constructor methods are introduced to make this cleaner.
- allow the subject to apply a change and inform the state change. This might be useful for in singleton design architecture where the user wants to do some operation on the universal state which then needs to be propagated to all the observers.

Just some thoughts for the future:
Currently, this is a binary but I would rather have this as a library. Even cooler transformation would be to a proc macro library, but that will be highly opinionated. Some real world use case would help shape the future of this repo as a proc macro lib.
