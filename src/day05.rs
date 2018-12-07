fn reduce(input: &str, remove: Option<char>) -> String {
    let mut current: String = input
        .chars()
        .filter(|&c| {
            remove
                .map(|to_remove| {
                    c.to_lowercase().to_string() != to_remove.to_lowercase().to_string()
                }).unwrap_or(true)
        }).collect();
    let mut made_changes = true;

    while made_changes {
        let chars = current.chars().collect::<Vec<_>>();
        made_changes = false;

        for idx in 0..chars.len() - 1 {
            let first = chars[idx];
            let second = chars[idx + 1];

            if first.to_lowercase().to_string() == second.to_lowercase().to_string()
                && first != second
            {
                current.remove(idx);
                current.remove(idx);
                made_changes = true;
                break;
            }
        }
    }

    current
}

pub fn star_one(input: &str) -> usize {
    let result = reduce(input, None);

    result.trim().len()
}

pub fn star_two(input: &str) -> usize {
    let possible_units = (b'a' as u32..=b'z' as u32)
        .flat_map(std::char::from_u32)
        .collect::<Vec<_>>();
    let results = possible_units
        .into_iter()
        .map(|c| reduce(input.trim(), Some(c)));

    results.map(|r| r.trim().len()).min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("dabAcCaCBAcCcaDA"), 4);
    }
}
