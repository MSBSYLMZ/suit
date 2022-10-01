use std::env;

use suit::generator::generator::generate;
fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let path = args.next().unwrap();
    generate(path).unwrap();
}
