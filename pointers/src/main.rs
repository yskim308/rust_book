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
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
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

    // a is a new countable reference to a list where
    // value : countable reference to value (clone)
    // list : Nil
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    // b is a new list where
    // value: mutable refcell of 3
    // list: the list a (which can have multiple references)
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    // same as b
    let c = List::Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // ssentially, every value in the list is a multi-referenced interior mutable value
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
