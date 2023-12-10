pub fn read_num(item: String) -> u32 {
    let mut num0 = 0;
    let mut num1 = 0;
    let mut find_one = false;

    for c in item.chars() {
        if c.is_ascii_digit() {
            let n = c.to_digit(10).unwrap();
            if !find_one {
                num0 = n;
                num1 = num0;
                find_one = true;
            } else {
                num1 = n;
            }
        }
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
}
