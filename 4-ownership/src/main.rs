fn main() {
    // s1 moved to s2, s1 invalidated
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");

    // to clone, use clone method on strings, expensive
    let s3 = String::from("clone");
    let s4 = s3.clone();
    println!("{s4}");

    // ownership of s2 moved to function, and goes out of scope
    takes_ownership(s2);

    // owernship given by return value
    let given = gives_ownership();
    println!("{given}");
}

fn takes_ownership(some_string: String) {
    // ownership of some_string transferred
    println!("'{some_string}' ownership transferred");
} // goes out of scope

// ownership given by function return value
fn gives_ownership() -> String {
    let some_string = String::from("yours now");
    some_string
}
