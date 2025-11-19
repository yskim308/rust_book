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

    // given passed as reference, ownership NOT transferred
    let length = calculate_length(&given);
    println!(" length of {given}:{length}");

    // to mutate the refernece, create mutable refernece and pass it
    let mut mutable_str = given;
    change_string(&mut mutable_str);
    println!("{mutable_str}");

    // slicing
    let literal = "hello world";
    let string_example = String::from("hello world");

    let literal_first = first_word(&literal);
    let string_first = first_word(&string_example);

    println!("{literal_first}");
    println!("{string_first}");
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

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", mutated");
}

fn first_word(s: &str) -> &str {
    // use string literals
    for (idx, &chr) in s.as_bytes().iter().enumerate() {
        if chr == b' ' {
            return &s[0..idx];
        }
    }
    &s[..]
}
