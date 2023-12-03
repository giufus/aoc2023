use std::fs::read_to_string;

mod one;

fn main() {
    one::one(read_to_string("input/parts/1").expect("missing input one"));
}
