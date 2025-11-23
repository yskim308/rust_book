mod fearless_concurrency;

use fearless_concurrency::threads;

fn main() {
    println!("-------------- Basic threads example: -------------");
    threads::run_basic_example();
    println!("-------------- thread ownership example: ----------");
    threads::run_ownership_example();
}
