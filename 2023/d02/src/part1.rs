use crate::consts::{MAX_BLUE, MAX_GREEN, MAX_RED};
use crate::game::{parse_games, GameInfo};

#[allow(unused)]
fn get_impossible_reason(i: &GameInfo) -> &str {
    if i.n_red() > MAX_RED {
        return "red cube is too much";
    }

    if i.n_green() > MAX_GREEN {
        return "green cube is too much";
    }

    if i.n_blue() > MAX_BLUE {
        return "blue cube is too much";
    }

    ""
}

pub fn solve<T: std::io::Read>(f: T) -> u32 {
    parse_games(f)
        .iter()
        .filter(|item| {
            let is_possible =
                item.n_red() <= MAX_RED && item.n_green() <= MAX_GREEN && item.n_blue() <= MAX_BLUE;
            // println!(
            //     "game {:?} is possible: {:?}, impossible reason: {:?}",
            //     item,
            //     is_possible,
            //     get_impossible_reason(item)
            // );
            is_possible
        })
        .map(|item| item.id())
        .sum()
}
