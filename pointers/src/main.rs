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

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("smart pointer with Hello World created");
    let c = CustomSmartPointer {
        data: String::from("hello world"),
    };
    drop(c);
    println!("smart pointer with hello world dropped before end of program");

    println!("smart poitner with integer 5 created");
    let _d = CustomSmartPointer { data: 5 };
}
