use crate::observer::Observer;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub trait Subject<T: Observer> {
    fn action<F: Fn(i32) -> i32>(&mut self, func: F);
    fn register(&mut self, obs: &Rc<RefCell<T>>);
    fn remove(&mut self, name: &str) -> Option<Weak<RefCell<T>>>;
}
