use d01::*;

#[test]
fn example2() {
    let f = utils::read_file_in_examples(env!("CARGO_MANIFEST_DIR"), "example2.txt").unwrap();
    let n = sum_all(f, d01::read_num::v2::read_num);
    assert_eq!(n, 281)
}
