use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"#\s*(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)").unwrap();
}

#[derive(Debug, Eq, PartialEq)]
pub struct Claim {
    id: usize,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl Claim {
    pub fn new(id: usize, left: usize, top: usize, width: usize, height: usize) -> Self {
        Self {
            id,
            top,
            left,
            width,
            height,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let groups = PATTERN.captures(input).unwrap();

        Self {
            id: groups[1].parse::<usize>().expect("Expected an id"),
            left: groups[2].parse::<usize>().expect("Expected a top value"),
            top: groups[3].parse::<usize>().expect("Expected a left value"),
            width: groups[4].parse::<usize>().expect("Expected a width"),
            height: groups[5].parse::<usize>().expect("Expected a height"),
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

pub fn star_one(input: &str) -> usize {
    let claims = input
        .lines()
        .filter(|l| l.len() > 0)
        .map(Claim::from_string);
    let mut coverage = HashMap::<(usize, usize), usize>::new();

    for claim in claims {
        for x in (claim.left + 1)..(claim.left + claim.width + 1) {
            for y in (claim.top + 1)..(claim.top + claim.height + 1) {
                let counter = coverage.entry((x, y)).or_insert(0);

                *counter += 1;
            }
        }
    }

    coverage
        .iter()
        .fold(0, |acc, (_, &count)| if count > 1 { acc + 1 } else { acc })
}

pub fn star_two(input: &str) -> usize {
    let claims: Vec<Claim> = input
        .lines()
        .filter(|l| l.len() > 0)
        .map(Claim::from_string)
        .collect();
    let mut coverage = HashMap::<(usize, usize), (usize, HashSet<usize>)>::new();

    for claim in &claims {
        for x in (claim.left + 1)..(claim.left + claim.width + 1) {
            for y in (claim.top + 1)..(claim.top + claim.height + 1) {
                let counter = coverage.entry((x, y)).or_insert((0, HashSet::new()));

                (*counter).0 += 1;
                (*counter).1.insert(claim.id);
            }
        }
    }

    let ids: Vec<_> = coverage
        .into_iter()
        .filter(|(_, (count, _))| count == &1)
        .map(|(_, (_, ids))| ids.into_iter().nth(0).unwrap())
        .collect();

    for claim in &claims {
        let area = claim.area();
        let coverage_for_id = ids.iter().filter(|id| *id == &claim.id).count();

        if area == coverage_for_id {
            return claim.id;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3)
    }

    #[test]
    fn test_claim_from_string() {
        assert_eq!(
            Claim::from_string("#123 @ 3,2: 5x4"),
            Claim::new(123, 3, 2, 5, 4)
        );
    }
}
