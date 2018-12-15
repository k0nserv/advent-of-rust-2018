#[derive(Debug)]
struct Elf {
    current_recipe: usize,
}

impl Elf {
    fn new(current_recipe: usize) -> Self {
        Self { current_recipe }
    }

    fn pick_new_recipe(&mut self, scoreboard: &[usize]) {
        self.current_recipe =
            (self.current_recipe + scoreboard[self.current_recipe] + 1) % scoreboard.len();
    }
}

fn make_new_recipes(elves: &[Elf], scoreboard: &[usize]) -> Vec<usize> {
    let sum: usize = elves.iter().map(|e| scoreboard[e.current_recipe]).sum();

    sum.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

pub fn star_one(recipes_to_make: usize) -> String {
    let mut scoreboard: Vec<usize> = vec![3, 7];
    let mut elves = vec![Elf::new(0), Elf::new(1)];

    while scoreboard.len() < recipes_to_make + 10 {
        let new_recipes = make_new_recipes(&elves, &scoreboard);
        scoreboard.extend(new_recipes);

        for elf in &mut elves {
            elf.pick_new_recipe(&scoreboard);
        }
    }

    let correction = scoreboard.len() - recipes_to_make - 10;
    scoreboard[scoreboard.len() - 10 - correction..scoreboard.len() - correction]
        .iter()
        .map(|d| d.to_string())
        .collect()
}

pub fn star_two(input: &[usize]) -> usize {
    let mut scoreboard: Vec<usize> = vec![3, 7];
    let mut elves = vec![Elf::new(0), Elf::new(1)];

    'outer: loop {
        let new_recipes = make_new_recipes(&elves, &scoreboard);
        for new_recipe in new_recipes {
            scoreboard.push(new_recipe);

            if scoreboard.len() >= input.len() {
                let sequence = &scoreboard[scoreboard.len() - input.len()..scoreboard.len()];

                if sequence == input {
                    break 'outer;
                }
            }
        }

        for elf in &mut elves {
            elf.pick_new_recipe(&scoreboard);
        }
    }

    scoreboard.len() - input.len()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(9), String::from("5158916779"));
        assert_eq!(star_one(5), String::from("0124515891"));
        assert_eq!(star_one(18), String::from("9251071085"));
        assert_eq!(star_one(2018), String::from("5941429882"));
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(&[5, 1, 5, 8, 9]), 9);
        assert_eq!(star_two(&[0, 1, 2, 4, 5]), 5);
        assert_eq!(star_two(&[9, 2, 5, 1, 0]), 18);
        assert_eq!(star_two(&[5, 9, 4, 1, 4]), 2018);
        assert_eq!(star_two(&[1, 2, 4, 5, 1, 5]), 6);
    }
}
