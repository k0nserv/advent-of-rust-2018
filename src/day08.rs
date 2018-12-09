use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type NodePointer = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    metadata: Vec<usize>,
    children: Vec<NodePointer>,
}

impl Node {
    fn new(child_count: usize, metadata_count: usize) -> Node {
        Self {
            metadata: Vec::<usize>::with_capacity(metadata_count),
            children: Vec::<NodePointer>::with_capacity(child_count),
        }
    }

    fn add_children(&mut self, new_children: Vec<NodePointer>) {
        self.children.extend(new_children);
    }

    fn add_metadata(&mut self, new_metadata: Vec<usize>) {
        self.metadata.extend(new_metadata);
    }

    fn metadata_sum(&self) -> usize {
        self.metadata.iter().sum()
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn value(&self) -> usize {
        if self.has_children() {
            self.metadata.iter().fold(0, |acc, &index| {
                if index <= self.children.len() {
                    acc + self.children[index - 1].borrow().value()
                } else {
                    acc
                }
            })
        } else {
            self.metadata_sum()
        }
    }

    fn recurse(numbers: &mut VecDeque<usize>) -> NodePointer {
        let child_count = numbers.pop_front().unwrap();
        let metadata_count = numbers.pop_front().unwrap();
        let node = Rc::new(RefCell::new(Self::new(child_count, metadata_count)));

        let children = (0..child_count)
            .map(|_| Self::recurse(numbers))
            .into_iter()
            .collect::<Vec<_>>();
        let metadata = (0..metadata_count)
            .map(|_| numbers.pop_front().unwrap())
            .collect::<Vec<_>>();

        node.borrow_mut().add_children(children);
        node.borrow_mut().add_metadata(metadata);

        node
    }

    fn traverse<F>(root: &NodePointer, mut f: F)
    where
        F: FnMut(&Self),
    {
        let mut to_visit: Vec<NodePointer> = vec![Rc::clone(root)];

        while !to_visit.is_empty() {
            let next = to_visit.pop().unwrap();
            f(&next.borrow_mut());

            for child in next.borrow().children.iter() {
                to_visit.push(Rc::clone(child));
            }
        }
    }
}

impl<'a> From<&'a str> for Node {
    fn from(input: &'a str) -> Self {
        let mut numbers = input
            .split_whitespace()
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .expect("Expected only parseable numbers")
            }).collect::<VecDeque<_>>();

        let root = Node::recurse(&mut numbers);

        Rc::try_unwrap(root)
            .expect("Expect exactly one reference to the root node")
            .into_inner()
    }
}

pub fn star_one(input: &str) -> usize {
    let tree = Rc::new(RefCell::new(Node::from(input)));
    let mut sum = 0;

    Node::traverse(&tree, |node: &Node| sum += node.metadata_sum());

    sum
}

pub fn star_two(input: &str) -> usize {
    let tree = Node::from(input);

    tree.value()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    static EXAMPLE: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE), 138)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(EXAMPLE), 66)
    }
}
