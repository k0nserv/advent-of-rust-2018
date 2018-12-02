use std::collections::{HashMap, HashSet};

fn almost_equal(lhs: &str, rhs: &str) -> Option<usize> {
    if lhs.len() != rhs.len() {
        return None;
    }

    let mut differs_by: Option<usize> = None;
    for (pos, (r, l)) in rhs.chars().zip(lhs.chars()).enumerate() {
        if r == l {
            continue;
        }

        if r != l && differs_by.is_some() {
            return None;
        }

        differs_by = Some(pos);
    }

    differs_by
}

pub fn star_one(input: &str) -> i64 {
    let counts = input
        .lines()
        .map(|id| {
            let mut map = HashMap::<char, usize>::new();

            id.chars().for_each(|c| {
                let counter = map.entry(c).or_insert(0);

                *counter += 1
            });

            let mut found_exactly_two = false;
            let mut found_exactly_three = false;

            let result = map.iter().fold((0, 0), |acc, (_, &count)| {
                if count == 3 && !found_exactly_three {
                    found_exactly_three = true;
                    (acc.0, acc.1 + 1)
                } else if count == 2 && !found_exactly_two {
                    found_exactly_two = true;
                    (acc.0 + 1, acc.1)
                } else {
                    acc
                }
            });
            result
        }).fold((0, 0), |acc, (two_count, three_count)| {
            (acc.0 + two_count, acc.1 + three_count)
        });

    counts.0 * counts.1
}

pub fn star_two(input: &str) -> String {
    let ids: Vec<_> = input
        .lines()
        .filter(|l| l.len() > 0)
        .map(String::from)
        .collect();
    let mut similar_ids = HashSet::<String>::new();
    let mut differ_by = None;

    for id in &ids {
        for inner_id in &ids {
            match almost_equal(id, inner_id) {
                None => continue,
                Some(pos) => {
                    similar_ids.insert(id.clone());
                    similar_ids.insert(inner_id.clone());
                    differ_by = Some(pos);
                    break;
                }
            }
        }
    }

    let mut first = similar_ids.iter().nth(0).unwrap().to_owned();
    first.remove(differ_by.unwrap());

    first
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
            12
        )
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(
                "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            ),
            "fgij"
        )
    }
}
