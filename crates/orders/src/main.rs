#![deny(clippy::all)]

use proto::{order::Order, warehouse::Warehouse};

fn main() {
    let x: usize = 5;
    println!("{}", x % 3);
}
