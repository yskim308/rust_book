use std::ops::Deref;

// ----- smart pointer implementation -----
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// ----- implementation to demonstrate drop ------
#[derive(Debug)]
struct CustomSmartPointer<T: std::fmt::Debug> {
    data: T,
}

impl<T: std::fmt::Debug> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {:?}", self.data);
    }
}

// ---- implementation to demonstrate Rc and RefCel -----
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// ===== implementation of tree ======
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // ---- box examples
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // ---- smart pointer with drop examples
    println!("smart pointer with Hello World created");
    let c = CustomSmartPointer {
        data: String::from("hello world"),
    };
    drop(c);
    println!("smart pointer with hello world dropped before end of program");

    println!("smart poitner with integer 5 created");
    let _d = CustomSmartPointer { data: 5 };

    // --- examples for Rc and RefCell -----
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // ====== examples for node/tree ===========
    let child = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let parent = Rc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&child)]),
    });

    *child.parent.borrow_mut() = Rc::downgrade(&parent);
}
