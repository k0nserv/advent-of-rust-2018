use std::collections::HashSet;
use std::iter;

#[derive(Debug)]
struct Rule {
    pattern: Vec<bool>,
    replacement: bool,
}

impl<'a> From<&'a str> for Rule {
    fn from(input: &'a str) -> Self {
        let parts = input.split("=>").collect::<Vec<&'a str>>();
        assert!(parts.len() == 2, "Each rule should have to parts");

        let pattern: Vec<bool> = parts[0].trim().chars().map(|c| c == '#').collect();
        assert!(
            pattern.len() == 5,
            "Each pattern should have exactly five parts"
        );

        let replacement = parts[1].trim().chars().map(|c| c == '#').nth(0).unwrap();

        Self {
            pattern,
            replacement,
        }
    }
}

impl Rule {
    fn matches(&self, part: &[bool]) -> bool {
        assert!(part.len() == self.pattern.len());
        part.iter().zip(self.pattern.iter()).all(|(a, b)| a == b)
    }
}

fn sum(state: &[bool], zero_point: usize, base_idx: i64) -> i64 {
    state.iter().enumerate().fold(0, |acc, (idx, planted)| {
        if !planted {
            acc
        } else {
            acc + (base_idx + idx as i64 - zero_point as i64)
        }
    })
}

fn parse(initial_state: &str, rules: &str, padding: usize) -> (Vec<Rule>, Vec<bool>) {
    let rules = rules
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(Rule::from)
        .collect();
    let mut state: Vec<bool> = initial_state.trim().chars().map(|c| c == '#').collect();

    for _ in 0..padding {
        state.insert(0, false);
        state.push(false);
    }

    (rules, state)
}

fn find_cycle(initial_state: Vec<bool>, rules: &[Rule]) -> usize {
    let mut state = initial_state;
    let mut observed_states = HashSet::<Vec<bool>>::new();

    for (i, _) in iter::repeat(0).enumerate() {
        state = next_generation(state, rules);

        let trimmed_pattern = state
            .iter()
            .skip_while(|&x| !x)
            .map(|&x| x.clone())
            .collect::<Vec<bool>>();

        if !observed_states.insert(trimmed_pattern) {
            return i;
        }
    }

    assert!(false, "If you are here something is definitely off");
    return 0; // This should never happen
}

fn next_generation(mut state: Vec<bool>, rules: &[Rule]) -> Vec<bool> {
    let state_size = state.len();

    state = state
        .iter()
        .enumerate()
        .map(|(id, _)| {
            if id == 0 || id == 1 || id == state_size - 1 || id == state_size - 2 {
                return false;
            }

            let part = &state[id - 2..id + 3];

            for rule in rules {
                if rule.matches(part) {
                    return rule.replacement;
                }
            }

            false
        }).collect();

    let mut to_append = vec![];

    if state[state.len() - 1] {
        to_append.push(false);
    }

    if state[state.len() - 2] {
        to_append.push(false);
    }

    if state[state.len() - 3] {
        to_append.push(false);
    }

    state.extend(to_append);

    state
}

pub fn star_one(initial_state: &str, rules: &str, padding: usize, num_generations: usize) -> i64 {
    let (parsed_rules, initial_parsed_state) = parse(initial_state, rules, padding);
    let mut state = initial_parsed_state;

    for _ in 0..num_generations {
        state = next_generation(state, &parsed_rules);
    }

    sum(&state, padding, 0)
}

pub fn star_two(initial_state: &str, rules: &str, padding: usize, num_generations: usize) -> i64 {
    let (parsed_rules, initial_parsed_state) = parse(initial_state, rules, padding);
    let mut state = initial_parsed_state.clone();
    let cycle_at = find_cycle(initial_parsed_state, &parsed_rules);
    let cycle_idx = cycle_at + (num_generations % cycle_at);

    for _ in 0..cycle_idx {
        state = next_generation(state, &parsed_rules);
    }

    sum(&state, padding, num_generations as i64 - cycle_idx as i64)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    static EXAMPLE_RULES: &str = "
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one("#..#.#..##......###...###", EXAMPLE_RULES, 3, 20),
            325
        );
    }
}
