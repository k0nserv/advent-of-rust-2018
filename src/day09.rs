use std::cell::RefCell;
use std::rc::Rc;

type NodePointer<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    value: T,
    next: Option<NodePointer<T>>,
    previous: Option<NodePointer<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> NodePointer<T> {
        let node = Rc::new(RefCell::new(Self {
            value,
            next: None,
            previous: None,
        }));

        {
            let mut mut_node = node.borrow_mut();

            mut_node.next = Some(node.clone());
            mut_node.previous = Some(node.clone());
        }

        node
    }

    fn next(&self) -> NodePointer<T> {
        Rc::clone(
            &self
                .next
                .as_ref()
                .expect("All nodes should have a next pointer"),
        )
    }

    fn previous(&self) -> NodePointer<T> {
        Rc::clone(
            &self
                .previous
                .as_ref()
                .expect("All nodes should have a next pointer"),
        )
    }

    fn value(&self) -> &T {
        &self.value
    }

    fn clockwise(&self, distance: usize) -> NodePointer<T> {
        let mut current: NodePointer<T> = Rc::clone(
            self.next
                .as_ref()
                .expect("All nodes should have a next pointer"),
        );

        for _ in 0..distance - 1 {
            current = {
                let borrowed = current.borrow();
                Rc::clone(
                    borrowed
                        .next
                        .as_ref()
                        .expect("All nodes should have a next pointer"),
                )
            }
        }

        current
    }

    fn counter_clockwise(&self, distance: usize) -> NodePointer<T> {
        let mut current: NodePointer<T> = Rc::clone(
            self.previous
                .as_ref()
                .expect("All nodes should have a next pointer"),
        );

        for _ in 0..distance - 1 {
            current = {
                let borrowed = current.borrow();
                Rc::clone(
                    borrowed
                        .previous
                        .as_ref()
                        .expect("All nodes should have a next pointer"),
                )
            };
        }

        current
    }

    fn remove(&mut self) -> &T {
        let previous: NodePointer<T> = Rc::clone(
            self.previous
                .as_ref()
                .expect("All nodes should have a next poiner"),
        );
        let next: NodePointer<T> = Rc::clone(
            self.next
                .as_ref()
                .expect("All nodes should have a next poiner"),
        );

        previous.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().previous = Some(previous);

        &self.value
    }

    fn insert_after(node: NodePointer<T>, value: T) -> NodePointer<T> {
        let next: NodePointer<T> = Rc::clone(
            node.borrow()
                .next
                .as_ref()
                .expect("All nodes should have a next poiner"),
        );
        let new = Self::new(value);

        node.borrow_mut().next = Some(Rc::clone(&new));
        next.borrow_mut().previous = Some(Rc::clone(&new));

        new.borrow_mut().previous = Some(Rc::clone(&node));
        new.borrow_mut().next = Some(Rc::clone(&next));

        new
    }
}

fn print(marbles: &[usize], current_idx: usize) -> String {
    let repr = marbles
        .iter()
        .enumerate()
        .map(|(idx, score)| {
            if idx == current_idx {
                format!("({}) ", score)
            } else {
                format!("{} ", score)
            }
        }).collect::<String>();

    repr
}

// This was efficient enough for part 1, but all the removes
// and inserts being O(n) on `Vec` was way too slow for part 2.
// Kept for historical purposes
pub fn solve(num_players: usize, last_marble_points: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut marbles = vec![0];
    let mut current_idx = 0;
    let mut current_player_idx = 0;
    marbles.reserve(last_marble_points);

    for marble_score in 1..last_marble_points + 1 {
        if marble_score % 23 != 0 {
            let insert_at = (current_idx + 1) % marbles.len();
            current_idx = insert_at + 1;
            marbles.insert(current_idx, marble_score); // O(n)

            assert!(marbles[current_idx] == marble_score);
        } else {
            scores[current_player_idx] += marble_score;
            let seven_counter_clockwise = match current_idx.overflowing_sub(7) {
                (idx, false) => idx,
                (overflowed_value, true) => {
                    marbles.len() - (usize::max_value() - overflowed_value + 1)
                }
            };

            scores[current_player_idx] += marbles.remove(seven_counter_clockwise); // O(n)

            current_idx = seven_counter_clockwise;
        }

        current_player_idx = (current_player_idx + 1) % scores.len();
    }

    scores.into_iter().max().unwrap()
}

pub fn solve_efficient(num_players: usize, last_marble_points: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut current_player_idx = 0;
    let mut current: NodePointer<usize> = Node::new(0);

    for marble_score in 1..last_marble_points + 1 {
        if marble_score % 23 != 0 {
            let node = current.borrow().clockwise(1);

            current = Node::insert_after(node, marble_score);

            assert!(current.borrow().value() == &marble_score);
        } else {
            scores[current_player_idx] += marble_score;
            let node = current.borrow().counter_clockwise(7);
            current = node.borrow().next();
            scores[current_player_idx] += node.borrow_mut().remove();
        }

        current_player_idx = (current_player_idx + 1) % scores.len();
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_one() {
        assert_eq!(solve(9, 25), 32);
        assert_eq!(solve(10, 1618), 8317);
        assert_eq!(solve(13, 7999), 146373);
        assert_eq!(solve(17, 1104), 2764);
        assert_eq!(solve(21, 6111), 54718);
        assert_eq!(solve(30, 5807), 37305);
    }

    #[test]
    fn test_star_one_efficient() {
        assert_eq!(solve_efficient(9, 25), 32);
        assert_eq!(solve_efficient(10, 1618), 8317);
        assert_eq!(solve_efficient(13, 7999), 146373);
        assert_eq!(solve_efficient(17, 1104), 2764);
        assert_eq!(solve_efficient(21, 6111), 54718);
        assert_eq!(solve_efficient(30, 5807), 37305);
    }
}
