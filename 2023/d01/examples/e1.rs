use d01::{read_num::v1::read_num, sum_all};
use utils::read_file_in_examples;

fn main() {
    let f = read_file_in_examples(env!("CARGO_MANIFEST_DIR"), "test1.txt").unwrap();
    println!("{}", sum_all(f, read_num));
}
