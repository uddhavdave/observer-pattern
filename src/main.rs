mod observer;
mod subject;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use observer::Observer;
use subject::Subject;

pub struct Watchdog {
    pub value: i32,
    pub name: String,
}

impl Observer for Watchdog {
    type State = i32;

    fn update(&mut self, data: Self::State) {
        self.value = data;
        println!("{} now sees {}", self.name, self.value);
    }

    fn new(name: &str, val: Self::State) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.into(),
            value: val,
        }))
    }
}

pub struct Bone {
    subj: i32,
    obs: Vec<Weak<RefCell<Watchdog>>>,
}

impl Bone {
    pub fn new(val: i32) -> Self {
        Self {
            subj: val,
            obs: vec![],
        }
    }
}

impl Subject<Watchdog> for Bone {
    fn action<F: Fn(i32) -> i32>(&mut self, func: F) {
        let state = func(self.subj);
        self.subj = state;
        self.obs.iter_mut().for_each(|ob| {
            if let Some(ob) = ob.upgrade() {
                ob.borrow_mut().update(state)
            }
        })
    }

    fn register(&mut self, obs: &Rc<RefCell<Watchdog>>) {
        self.obs.push(Rc::downgrade(obs))
    }

    fn remove(&mut self, name: &str) -> Option<Weak<RefCell<Watchdog>>> {
        let mut index: Option<usize> = None;

        for (i, observer) in self.obs.iter().enumerate() {
            if observer
                .upgrade()
                .map_or(false, |obs| obs.borrow().name == name)
            {
                index = Some(i);
                break;
            }
        }

        println!("Removed: {name}");

        if let Some(index) = index {
            Some(self.obs.remove(index))
        } else {
            None
        }
    }
}

fn main() {
    let mut subject = Bone::new(3);
    let watchdog = Watchdog::new("tommy", 3);
    let watchdog1 = Watchdog::new("cookie", 3);
    let watchdog2 = Watchdog::new("rocko", 3);

    subject.register(&watchdog);
    subject.register(&watchdog1);
    subject.register(&watchdog2);

    // Change subject state
    subject.action(|mut sub: i32| {
        sub += 1;
        sub
    });

    // Remove one of the observers
    // Note: we are using name as the identifier here
    _ = subject.remove("tommy");

    // Make changes in another observer
    watchdog2.borrow_mut().name = "something".into();
    println!("rocko changed name to something");

    for _ in 0..10 {
        subject.action(|mut sub: i32| {
            sub += 1;
            println!("Subject is now {sub}");
            sub
        });
    }
}
