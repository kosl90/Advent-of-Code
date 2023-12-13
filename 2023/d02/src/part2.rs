use crate::game::parse_games;

pub fn solve<T: std::io::Read>(f: T) -> u32 {
    parse_games(f)
        .iter()
        .map(|item| match (item.n_red(), item.n_blue(), item.n_green()) {
            (0, 0, 0) => 0,
            (0, 0, n) => n,
            (0, n, 0) => n,
            (n, 0, 0) => n,
            (0, n1, n2) => n1 * n2,
            (n1, 0, n2) => n1 * n2,
            (n1, n2, 0) => n1 * n2,
            (n1, n2, n3) => n1 * n2 * n3,
        })
        .sum()
}
