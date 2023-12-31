use d02::part1::solve;
use utils::read_file_in_examples;

fn main() {
    let file = read_file_in_examples(env!("CARGO_MANIFEST_DIR"), "test1.txt").unwrap();
    println!("id sum: {}", solve(file));
}
