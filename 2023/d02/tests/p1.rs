use d02::part1::solve;
use utils::read_file_in_examples;

#[test]
fn it_works() {
    let f = read_file_in_examples(env!("CARGO_MANIFEST_DIR"), "p1.txt").unwrap();
    let result = solve(f);
    assert_eq!(result, 8);
}
