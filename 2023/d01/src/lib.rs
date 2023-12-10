use std::io::{BufRead, BufReader};

pub mod read_num;

pub fn sum_all<T, Fn>(reader: T, read_num: Fn) -> u32
where
    T: std::io::Read,
    Fn: FnMut(String) -> u32,
{
    let buf = BufReader::new(reader);
    let nums = buf
        .lines()
        .take_while(|item| item.is_ok())
        .map(|item| item.unwrap())
        .map(read_num)
        .collect::<Vec<_>>();
    // dbg!("{:?}", &nums);
    nums.iter().sum()
}
