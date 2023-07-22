use std::hash::Hash;
/// Subject Trait provides methods for management of observers
/// and updation mechanism for each of them.
///
/// Trait uses a callback function to notify observers on the latest
/// state value.
pub trait Subject<T: Clone, U: Hash + Eq> {
    //
    type Callback: Fn(T);

    fn action<F: FnMut(T) -> T>(&mut self, func: F);
    fn register(&mut self, obs: Self::Callback, name: U);
    fn remove(&mut self, name: U) -> Option<Self::Callback>;
}
