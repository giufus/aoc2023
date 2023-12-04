use std::fs::read_to_string;

mod one;

fn main() {
    one::naive_1(read_to_string("input/parts/1.1").expect("missing input one"));
    one::run(read_to_string("input/parts/1.2").expect("missing input 2").as_str());
}
