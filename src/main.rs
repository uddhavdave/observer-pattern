mod subject;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use subject::Subject;

pub struct Dog {
    pub value: i32,
    pub name: String,
}

impl Dog {
    fn new(name: &str, val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.into(),
            value: val,
        }))
    }
}

pub struct Bone {
    subj: i32,
    obs: HashMap<String, Box<dyn Fn(i32)>>,
}

impl Bone {
    pub fn new(val: i32) -> Self {
        Self {
            subj: val,
            obs: HashMap::new(),
        }
    }
}

impl Subject<i32, &str> for Bone {
    type Callback = Box<dyn Fn(i32)>;

    fn action<F: FnMut(i32) -> i32>(&mut self, mut func: F) {
        let state = func(self.subj);
        self.subj = state;
        self.obs.iter_mut().for_each(|(_id, ob)| ob(state))
    }

    fn register(&mut self, obs: Self::Callback, name: &str) {
        self.obs.insert(name.into(), obs);
    }

    fn remove(&mut self, name: &str) -> Option<Self::Callback> {
        println!("Removed: {name}");
        self.obs.remove(name)
    }
}

fn main() {
    let mut subject = Bone::new(3);
    let dog = Dog::new("tommy", 3);
    let dog1 = Dog::new("cookie", 3);
    let dog2 = Dog::new("rocko", 3);

    let dog_clone = dog.clone();
    let notify_dog = move |x| {
        let mut dog = dog_clone.borrow_mut();
        dog.value = x;
        println!("{} now sees {}", dog.name, dog.value);
    };

    let dog1_clone = dog1.clone();
    let notify_dog1 = move |x| {
        let mut dog = dog1_clone.borrow_mut();
        dog.value = x;
        println!("{} now sees {}", dog.name, x);
    };

    let dog2_clone = dog2.clone();
    let notify_dog2 = move |x| {
        let mut dog = dog2_clone.borrow_mut();
        dog.value = x;
        println!("{} now sees {}", dog.name, x);
    };

    subject.register(Box::new(notify_dog), &dog.borrow().name);
    subject.register(Box::new(notify_dog1), &dog1.borrow().name);
    subject.register(Box::new(notify_dog2), &dog2.borrow().name);

    // Change subject state
    subject.action(|mut sub: i32| {
        sub += 1;
        sub
    });

    // Remove one of the observers
    // Note: we are using name as the identifier here
    _ = subject.remove("tommy");

    // Make changes in another observer
    dog2.borrow_mut().name = "koko".into();
    println!("rocko changed name to koko");

    // Do 10 more changes
    for _ in 0..10 {
        subject.action(|mut sub: i32| {
            sub += 1;
            println!("\nSubject is now {sub}");
            sub
        });
    }
}
