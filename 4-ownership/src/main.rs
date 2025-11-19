fn main() {
    // s1 moved to s2, s1 invalidated
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");
}
