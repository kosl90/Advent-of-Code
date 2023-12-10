fn consume_spell_num_if_possible(st: &str) -> Option<(u32, usize)> {
    if st.len() >= 5 {
        let s = &st[0..5];
        if "three".eq(s) {
            return Some((3, 5));
        } else if "seven".eq(s) {
            return Some((7, 5));
        } else if "eight".eq(s) {
            return Some((8, 5));
        }
    }

    if st.len() >= 4 {
        let s = &st[0..4];
        if "four".eq(s) {
            return Some((4, 4));
        } else if "five".eq(s) {
            return Some((5, 4));
        } else if "nine".eq(s) {
            return Some((9, 4));
        }
    }

    if st.len() >= 3 {
        let s = &st[0..3];
        if s.eq("one") {
            return Some((1, 3));
        } else if s.eq("two") {
            return Some((2, 3));
        } else if s.eq("six") {
            return Some((6, 3));
        }
    }

    None
}

pub fn read_num(item: String) -> u32 {
    let mut num0 = 0;
    let mut num1 = 0;
    let mut find_one = false;

    let mut i = 0;
    let bytes = item.as_bytes();
    while i < item.len() {
        let c = bytes[i];

        if c.is_ascii_digit() {
            let n = item[i..i + 1].parse::<u32>().unwrap();
            if !find_one {
                num0 = n;
                num1 = num0;
                find_one = true;
            } else {
                num1 = n;
            }
        } else if let Some((num, _)) = consume_spell_num_if_possible(&item[i..]) {
            if !find_one {
                num0 = num;
                num1 = num0;
                find_one = true;
            } else {
                num1 = num;
            }
        }

        i += 1;
    }

    num0 * 10 + num1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_num() {
        assert_eq!(read_num("1abc2".into()), 12);
        assert_eq!(read_num("pqr3stu8vwx".into()), 38);
    }

    #[test]
    fn multiple_num() {
        assert_eq!(read_num("a1b2c3d4e5f".into()), 15);
    }

    #[test]

    fn one_num() {
        assert_eq!(read_num("treb7uchet".into()), 77);
    }

    #[test]
    fn zero_num() {
        assert_eq!(read_num("abc".into()), 0);
    }

    #[test]
    fn spell_num() {
        assert_eq!(read_num("two1nine".into()), 29);
        assert_eq!(read_num("eightwothree".into()), 83);
        assert_eq!(read_num("abcone2threexyz".into()), 13);
        assert_eq!(read_num("xtwone3four".into()), 24);
        assert_eq!(read_num("4nineeightseven2".into()), 42);
        assert_eq!(read_num("zoneight234".into()), 14);
        assert_eq!(read_num("7pqrstsixteen".into()), 76);
    }
}
