mod fearless_concurrency;

use fearless_concurrency::threads;

fn main() {
    println!("Hello, world!");
    threads::hello_world();
}
