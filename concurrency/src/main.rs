mod fearless_concurrency;
mod messaging;

use fearless_concurrency::threads;
use messaging::channel;

fn main() {
    println!("-------------- Basic threads example: -------------");
    threads::run_basic_example();
    println!("-------------- thread ownership example: ----------");
    threads::run_ownership_example();

    println!("-------------- basic channel example: --------------");
    channel::run_basic_example();
}
