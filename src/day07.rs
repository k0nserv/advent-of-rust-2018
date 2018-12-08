use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug)]
struct Step {
    id: char,
    pub unlock_count: usize,
    required_by: Vec<Rc<RefCell<Step>>>,
}

impl Step {
    fn new(id: char) -> Self {
        Self {
            id,
            unlock_count: 0,
            required_by: vec![],
        }
    }

    fn unlock(&mut self) {
        if self.unlock_count > 0 {
            self.unlock_count -= 1;
        }
    }

    fn set_unlock_count(&mut self, count: usize) {
        self.unlock_count = count;
    }

    fn is_unlocked(&self) -> bool {
        self.unlock_count == 0
    }

    fn required_by(&self) -> &Vec<Rc<RefCell<Step>>> {
        &self.required_by
    }

    fn add_required_by(&mut self, required_by: Rc<RefCell<Step>>) {
        self.required_by.push(required_by);
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Step {}

fn parse(input: &str) -> Vec<Rc<RefCell<Step>>> {
    let clean_lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .filter(|line| line.starts_with("Step "))
        .collect::<Vec<_>>();

    let mappings: Vec<(char, char)> = clean_lines
        .iter()
        .map(|line| {
            let id = line
                .trim_start_matches("Step ")
                .chars()
                .nth(0)
                .expect("Expected to find ids");

            let pos = line.rfind(" can begin.").expect(&format!(
                "Expected the string ` can begin.` in {}, but found nothing",
                line
            ));

            let id2 = line.chars().nth(pos - 1).unwrap();

            (id, id2)
        }).collect();
    let ids = mappings
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<HashSet<_>>();
    let steps = mappings
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .map(|&id| (id, Rc::new(RefCell::new(Step::new(id)))))
        .collect::<HashMap<_, _>>();

    let mut no_requirments = ids.clone();

    mappings.iter().for_each(|(id, required_by_id)| {
        no_requirments.remove(&required_by_id);
        let other_step = {
            Rc::clone(steps.get(&required_by_id).expect(&format!(
                "Expected existing step for id: {}",
                required_by_id
            )))
        };

        if let Some(step) = steps.get(&id) {
            step.borrow_mut().add_required_by(other_step);
        }
    });

    steps.values().for_each(|value| {
        let requires_count = steps.values().fold(0, |acc, other_value| {
            if value == other_value {
                return acc;
            }

            if other_value.borrow().required_by().contains(value) {
                return acc + 1;
            }

            return acc;
        });

        value.borrow_mut().set_unlock_count(requires_count);
    });

    let mut firsts = no_requirments
        .into_iter()
        .map(|id| steps.get(&id).unwrap().clone())
        .collect::<Vec<_>>();
    firsts.sort_by(|a, b| b.cmp(a));

    firsts
}

pub fn star_one(input: &str) -> String {
    let mut first_steps = parse(input);
    first_steps.sort_by(|a, b| b.cmp(a));
    let mut stack = vec![];
    for step in first_steps {
        stack.push(step);
    }
    let mut result = String::new();

    while !stack.is_empty() {
        let next = stack.pop().unwrap();
        result.push(next.borrow().id);

        for to_explore in next.borrow().required_by() {
            to_explore.borrow_mut().unlock();

            if !stack.contains(&to_explore) && to_explore.borrow().is_unlocked() {
                stack.push(to_explore.clone());
            }
        }

        stack.sort_by(|a, b| b.cmp(a));
    }

    result
}

pub fn star_two(input: &str, num_workers: usize, base_time: usize) -> i64 {
    let mut first_steps = parse(input);
    first_steps.sort_by(|a, b| b.cmp(a));
    let mut stack = vec![];
    for step in first_steps {
        stack.push(step);
    }
    let mut result = String::new();
    let mut time_taken = 0;
    let mut busy_counters: Vec<(Option<Rc<RefCell<Step>>>, usize)> = vec![(None, 0); num_workers];

    while !stack.is_empty() || busy_counters.iter().any(|&(_, value)| value > 0) {
        busy_counters = busy_counters
            .into_iter()
            .map(|(potential_step, x)| match x.overflowing_sub(1) {
                (new_value, false) => {
                    if new_value == 0 {
                        if let Some(step) = potential_step {
                            for to_explore in step.borrow().required_by() {
                                to_explore.borrow_mut().unlock();

                                if !stack.contains(&to_explore) && to_explore.borrow().is_unlocked()
                                {
                                    stack.push(to_explore.clone());
                                }
                            }
                        }
                        (None, 0)
                    } else {
                        (potential_step, new_value)
                    }
                }
                (_, true) => (None, 0),
            }).collect();
        stack.sort_by(|a, b| b.cmp(a));

        let available_worker_ids: Vec<_> = busy_counters
            .iter()
            .enumerate()
            .filter(|(_, &(_, value))| value == 0)
            .map(|(id, _)| id.clone())
            .collect();

        available_worker_ids.iter().for_each(|id| {
            if stack.is_empty() {
                return;
            }

            let next = stack.pop().unwrap();
            result.push(next.borrow().id);

            let work_time = base_time + (next.borrow().id as u32 - 64) as usize;
            busy_counters[*id] = (Some(Rc::clone(&next)), work_time);
        });

        time_taken += 1;
    }

    println!("{}", result);

    time_taken - 1
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    static EXAMPLE: &'static str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE), "CABDFE");
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(EXAMPLE, 2, 0), 15);
    }
}
