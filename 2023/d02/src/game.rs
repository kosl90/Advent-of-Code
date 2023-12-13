use std::io::{BufRead, BufReader};

use crate::parser::{eat, eat_while, eat_whitespace};

#[derive(Debug)]
pub struct GameInfo {
    id: u32,
    n_red: u32,
    n_green: u32,
    n_blue: u32,
}

impl std::cmp::PartialEq for GameInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.n_red == other.n_red
            && self.n_green == other.n_green
            && self.n_blue == other.n_blue
    }
}

impl GameInfo {
    pub fn new(id: u32, n_red: u32, n_green: u32, n_blue: u32) -> Self {
        GameInfo {
            id,
            n_red,
            n_green,
            n_blue,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn n_red(&self) -> u32 {
        self.n_red
    }
    pub fn n_green(&self) -> u32 {
        self.n_green
    }
    pub fn n_blue(&self) -> u32 {
        self.n_blue
    }
}

#[derive(Default, Debug, Clone)]
pub struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl std::cmp::PartialEq for GameSet {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl std::ops::Add<GameSet> for GameSet {
    type Output = GameSet;

    fn add(self, rhs: GameSet) -> GameSet {
        GameSet::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl std::ops::Add<&GameSet> for GameSet {
    type Output = GameSet;

    fn add(self, rhs: &GameSet) -> GameSet {
        GameSet::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}
impl std::ops::Add<&GameSet> for &GameSet {
    type Output = GameSet;
    fn add(self, rhs: &GameSet) -> GameSet {
        GameSet::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl std::ops::AddAssign<&GameSet> for &mut GameSet {
    fn add_assign(&mut self, rhs: &GameSet) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl GameSet {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        GameSet { red, green, blue }
    }
}

fn parse_game_header(game: &str) -> (u32, &str) {
    let game = eat(game, "Game").unwrap();
    let (_, game) = eat_whitespace(game);
    let (a, game) = eat_while(game, |item| item.is_ascii_digit());
    let id: u32 = a.parse().unwrap();
    let game = eat(game, ":").unwrap();
    let (_, game) = eat_whitespace(game);
    (id, game)
}

pub enum Color {
    Red,
    Green,
    Blue,
    None,
}

pub fn eat_color(game: &str) -> (Color, &str) {
    if let Some(rest) = game.strip_prefix("red") {
        return (Color::Red, rest);
    } else if let Some(rest) = game.strip_prefix("green") {
        return (Color::Green, rest);
    } else if let Some(rest) = game.strip_prefix("blue") {
        return (Color::Blue, rest);
    }

    (Color::None, game)
}

pub fn parse_dice(game: &str) -> GameSet {
    // println!("parse game set element: {:?}", game);
    let (_, game) = eat_whitespace(game);
    let (n, game) = eat_while(game, |item| item.is_ascii_digit());
    let n = n.parse().unwrap();
    let (_, game) = eat_whitespace(game);
    let (color, _) = eat_color(game);
    match color {
        Color::Red => GameSet::new(n, 0, 0),
        Color::Green => GameSet::new(0, n, 0),
        Color::Blue => GameSet::new(0, 0, n),
        _ => GameSet::default(),
    }
}

pub fn parse_game_set(game: &str) -> Option<GameSet> {
    let (_, game) = eat_whitespace(game);
    let mut total = GameSet::default();
    game.split(',')
        .map(parse_dice)
        .fold(&mut total, |mut total, item| {
            total += &item;
            total
        });
    // println!("game set: {:?}", total);
    Some(total)
}

pub fn parse_game_sets(game: &str) -> Vec<GameSet> {
    // println!("parse game sets: {}", game);
    game.split(';')
        .map(|item| parse_game_set(item).unwrap())
        .collect()
}

pub fn parse_game(game: String) -> GameInfo {
    // println!("parse game: {:?}", game);
    let (id, rest) = parse_game_header(&game);
    // println!("GameHeader: {:?}, {:?}", id, rest);
    let game_sets = parse_game_sets(rest);
    // println!("game sets: {:?}", game_sets);
    let mut total_game_set = GameSet::default();
    game_sets.iter().fold(&mut total_game_set, |acc, x| {
        if acc.red < x.red {
            acc.red = x.red
        }
        if acc.blue < x.blue {
            acc.blue = x.blue
        }
        if acc.green < x.green {
            acc.green = x.green
        }
        acc
    });

    // println!("max game set: {:?}", total_game_set);

    GameInfo::new(
        id,
        total_game_set.red,
        total_game_set.green,
        total_game_set.blue,
    )
}

pub fn parse_games<F: std::io::Read>(f: F) -> Vec<GameInfo> {
    let reader = BufReader::new(f);
    reader
        .lines()
        .take_while(|item| item.is_ok())
        .map(|item| item.unwrap())
        .map(parse_game)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2green, 6 blue; 2 green".to_string()),
            GameInfo::new(1, 4, 2, 6)
        )
    }

    #[test]
    fn test_parse_game_set() {
        assert_eq!(
            parse_game_set("3 blue, 4 red").unwrap(),
            GameSet::new(4, 0, 3)
        );
    }

    #[test]
    fn test_parse_game_sets() {
        assert_eq!(
            parse_game_sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            vec![
                GameSet::new(4, 0, 3),
                GameSet::new(1, 2, 6),
                GameSet::new(0, 2, 0)
            ]
        )
    }
}
