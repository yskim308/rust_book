fn main() {
    // s1 moved to s2, s1 invalidated
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");

    // to clone, use clone method on strings, expensive
    let s3 = String::from("clone");
    let s4 = s3.clone();
    println!("{s4}");

    takes_ownership(s2);

    // println!("{s2}");
}

fn takes_ownership(some_string: String) {
    // ownership of some_string transferred
    println!("'{some_string}' ownership transferred");
} // goes out of scope
