pub fn eat_whitespace(game: &str) -> (&str, &str) {
    eat_while(game, |item| item.is_whitespace())
}

pub fn eat_while<F>(game: &str, f: F) -> (&str, &str)
where
    F: Fn(char) -> bool,
{
    let mut i = 0;
    for (idx, c) in game.char_indices() {
        if !f(c) {
            i = idx;
            break;
        }
        i = idx;
    }

    if i >= game.len() {
        (game, "")
    } else {
        (&game[..i], &game[i..])
    }
}

pub fn eat<'a>(game: &'a str, a: &'a str) -> Option<&'a str> {
    if game.len() < a.len() {
        return None;
    }

    game.strip_prefix(a)
}
