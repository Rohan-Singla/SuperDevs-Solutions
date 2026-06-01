use std::{cell::RefCell, ops::Deref, rc::Rc};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn list_sum(mylist: &List) -> i32 {
    match mylist {
        List::Cons(val, next) => val + list_sum(next),

        List::Nil => 0,
    }
}

pub struct Wrapper<T>(T);

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn double_len(s: &str) -> usize {
    s.len() * 2
}

pub fn count_owners(n: usize) -> usize {
    let mut clones = vec![];

    let string = Rc::new(String::from("Shared"));

    let mut count = 0;

    for _ in 0..n {
        let newclone = Rc::clone(&string);

        clones.push(newclone);
    }

    count = Rc::strong_count(&string);

    return count;
}

pub struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

pub fn count_to(n: i32) -> i32 {
    let counter = Counter::new();
    for _ in 0..n {
        counter.increment();
    }
    counter.get()
}
