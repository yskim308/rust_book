mod fearless_concurrency;

use fearless_concurrency::{channel, threads};

fn main() {
    println!("-------------- Basic threads example: -------------");
    threads::run_basic_example();
    println!("-------------- thread ownership example: ----------");
    threads::run_ownership_example();

    println!("-------------- basic channel example: --------------");
    channel::run_basic_example();
}
