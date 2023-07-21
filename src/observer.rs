use std::{cell::RefCell, rc::Rc};

pub trait Observer {
    type State;
    fn update(&mut self, data: Self::State);
    fn new(name: &str, val: Self::State) -> Rc<RefCell<Self>>;
}
