enum List {
    Cons(Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn run() {
    println!("[Smart pointers]");

    // Box for recursive types
    let _list = Cons(Box::new(Cons(Box::new(Nil))));

    // Deref trait
    let x = 5;
    let x_ref = &x;
    let x_box = Box::new(x);
    let x_struct = StructWithDeref::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *x_ref);
    assert_eq!(5, *x_box);
    assert_eq!(5, *x_struct);
}

struct StructWithDeref<T> {
    data: Box<T>,
}

impl<T> StructWithDeref<T> {
    fn new(x: T) -> StructWithDeref<T> {
        StructWithDeref { data: Box::new(x) }
    }
}

impl<T> std::ops::Deref for StructWithDeref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
