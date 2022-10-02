use std::env::{self, Args};

use suit::generator::generator::generate;
fn main() {
    let args = env::args();
    run(args);
}

fn run(mut args: Args) {
    args.next().unwrap();
    let path = args.next().unwrap();
    generate(path).unwrap();
}
