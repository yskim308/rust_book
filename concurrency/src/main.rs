mod asynchronous_programming;
mod fearless_concurrency;

use asynchronous_programming::asynchronous;
use fearless_concurrency::{channel, mutex, threads};

fn main() {
    println!("\n-------------- Basic threads example: -------------");
    threads::run_basic_example();
    println!("-------------- thread ownership example: ----------");
    threads::run_ownership_example();

    println!("\n-------------- basic channel example: --------------");
    channel::run_basic_example();

    println!("\n-------------- basic mutex example: ---------------");
    mutex::run_basic_example();

    println!("\n--------------- async examples -------------------");
    asynchronous::run_basic_example();
    println!("----------------- async channel example -----------");
    asynchronous::run_channel_example();
}
