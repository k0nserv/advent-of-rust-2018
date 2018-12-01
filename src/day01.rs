use std::collections::HashSet;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = i64> + 'a {
    input
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|n| n.trim())
        .filter(|n| n.len() > 1)
        .map(|number| number.parse::<i64>().expect("Expected only valid numbers"))
}

pub fn star_one(input: &str) -> i64 {
    parse(input).fold(0, |acc, x| acc + x)
}

pub fn star_two(input: &str) -> i64 {
    let instructions = parse(input).collect::<Vec<_>>();

    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(0);
    let mut current_value = 0;
    let mut idx = 0;

    loop {
        let instruction = instructions[idx % instructions.len()];
        current_value += instruction;

        if seen_frequencies.contains(&current_value) {
            break current_value;
        }

        seen_frequencies.insert(current_value);
        idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("+1, -2, +3, +1"), 3);
        assert_eq!(star_one("+1, +1, +1"), 3);
        assert_eq!(star_one("+1, +1, -2"), 0);
        assert_eq!(star_one("-1, -2, -3"), -6);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("+1, -1"), 0);
        assert_eq!(star_two("+3, +3, +4, -2, -4"), 10);
        assert_eq!(star_two("-6, +3, +8, +5, -6"), 5);
        assert_eq!(star_two("+7, +7, -2, -7, -4"), 14);
    }
}
