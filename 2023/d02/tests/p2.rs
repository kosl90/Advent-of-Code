use d02::part2::solve;
use utils::read_file_in_examples;

#[test]
fn test_p2() {
    let f = read_file_in_examples(env!("CARGO_MANIFEST_DIR"), "p1.txt").unwrap();
    assert_eq!(solve(f), 2286);
}
