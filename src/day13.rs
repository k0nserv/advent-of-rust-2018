use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use std::cell::RefCell;
use std::rc::Rc;

type Location = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn parse(input: char) -> Option<Self> {
        match input {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }

    fn along(&self, location: &Location) -> Location {
        let (x, y) = location.clone();
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    fn counter_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone, Debug)]
enum Action {
    TurnLeft,
    Continue,
    TurnRight,
}

impl Action {
    fn next(&self) -> Self {
        match self {
            Action::TurnLeft => Action::Continue,
            Action::Continue => Action::TurnRight,
            Action::TurnRight => Action::TurnLeft,
        }
    }

    fn new_direction(&self, current_direction: &Direction) -> Direction {
        match self {
            Action::TurnLeft => current_direction.counter_clockwise(),
            Action::Continue => current_direction.clone(),
            Action::TurnRight => current_direction.clockwise(),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::TurnLeft
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TrackType {
    Horizontal,   // -
    Vertical,     // |
    Curve1,       // /
    Curve2,       // \
    Intersection, // +
}

impl TrackType {
    fn parse(input: char) -> Option<Self> {
        match input {
            '-' => Some(TrackType::Horizontal),
            '|' => Some(TrackType::Vertical),
            '/' => Some(TrackType::Curve1),
            '\\' => Some(TrackType::Curve2),
            '+' => Some(TrackType::Intersection),
            '^' => Some(TrackType::Vertical),
            '>' => Some(TrackType::Horizontal),
            'v' => Some(TrackType::Vertical),
            '<' => Some(TrackType::Horizontal),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            TrackType::Horizontal => '-',
            TrackType::Vertical => '|',
            TrackType::Curve1 => '/',
            TrackType::Curve2 => '\\',
            TrackType::Intersection => '+',
        }
    }
}

#[derive(Clone, Debug)]
struct Cart {
    current_direction: Direction,
    current_action: Action,
    is_alive: bool,
}

impl Cart {
    fn new(direction: Direction) -> Self {
        Self {
            current_direction: direction,
            current_action: Action::default(),
            is_alive: true,
        }
    }

    fn advance(&mut self) {
        let new_direction = self.current_action.new_direction(&self.current_direction);
        self.current_action = self.current_action.next();
        self.current_direction = new_direction;
    }

    fn change_direction(&mut self, new_direction: Direction) {
        self.current_direction = new_direction;
    }
}

struct Track {
    grid: Vec<Vec<Option<TrackType>>>,
    carts: HashMap<Location, Vec<Rc<RefCell<Cart>>>>,
}

impl Track {
    fn has_crash(&self) -> bool {
        self.carts.iter().any(|(_, carts)| carts.len() > 1)
    }

    fn crash_location(&self) -> Option<Location> {
        if !self.has_crash() {
            None
        } else {
            let collisions = self
                .carts
                .iter()
                .filter(|(_, carts)| carts.len() > 1)
                .collect::<Vec<_>>();

            assert!(
                collisions.len() == 1,
                "Expected one collision found {} in {:?}",
                collisions.len(),
                collisions
            );
            collisions
                .into_iter()
                .nth(0)
                .map(|(location, _)| location.clone())
        }
    }

    fn num_alive_carts(&self) -> usize {
        self.carts.values().fold(0, |outer_acc, carts| {
            outer_acc + carts.iter().fold(
                0,
                |acc, cart| if cart.borrow().is_alive { acc + 1 } else { acc },
            )
        })
    }

    fn alive_carts_locations(&self) -> Vec<Location> {
        self.carts
            .iter()
            .flat_map(|(&location, carts)| {
                let cloned_location = location.clone();

                if carts.iter().any(|cart| cart.borrow().is_alive) {
                    Some(cloned_location)
                } else {
                    None
                }
            }).collect()
    }

    fn tick(&mut self, halt_on_collision: bool) {
        let mut order = self
            .carts
            .iter()
            .filter(|(_, carts)| carts.iter().any(|c| c.borrow().is_alive))
            .map(|(x, _)| x.clone())
            .collect::<Vec<Location>>();
        order.sort_by(|a, b| {
            let order = a.0.cmp(&b.0);
            if order != Ordering::Equal {
                order
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut new_carts = self.carts.clone();

        'outer: for location in order {
            let (x, y) = location;
            let carts = self.carts.get(&location).unwrap().clone();
            let track_type = &self.grid[y][x];

            for cart in carts.iter() {
                if !cart.borrow().is_alive {
                    continue;
                }

                let (did_collide, new_location) = match track_type {
                    Some(TrackType::Intersection) => {
                        let new_direction = cart
                            .borrow()
                            .current_action
                            .new_direction(&cart.borrow().current_direction);
                        let new_location = new_direction.along(&location);
                        let entry = new_carts.entry(new_location).or_insert(vec![]);
                        cart.borrow_mut().advance();
                        entry.push(Rc::clone(cart));
                        let did_collide = entry.iter().filter(|c| c.borrow().is_alive).count() > 1;

                        if did_collide {
                            entry.iter().for_each(|c| c.borrow_mut().is_alive = false);
                        }

                        (did_collide, new_location)
                    }
                    Some(TrackType::Horizontal) | Some(TrackType::Vertical) => {
                        assert!(
                            ((track_type == &Some(TrackType::Horizontal)
                                && (cart.borrow().current_direction == Direction::Left
                                    || cart.borrow().current_direction == Direction::Right))
                                || track_type == &Some(TrackType::Vertical)
                                    && (cart.borrow().current_direction == Direction::Up
                                        || cart.borrow().current_direction == Direction::Down))
                        );

                        let new_location = cart.borrow().current_direction.along(&location);
                        let entry = new_carts.entry(new_location).or_insert(vec![]);
                        entry.push(cart.clone());
                        let did_collide = entry.iter().filter(|c| c.borrow().is_alive).count() > 1;

                        if did_collide {
                            entry.iter().for_each(|c| c.borrow_mut().is_alive = false);
                        }

                        (did_collide, new_location)
                    }
                    Some(TrackType::Curve1) => {
                        // /
                        let new_direction = match cart.borrow().current_direction {
                            // /
                            // |
                            Direction::Up => Direction::Right,

                            // -/
                            Direction::Right => Direction::Up,

                            // |
                            // /
                            Direction::Down => Direction::Left,

                            // /--
                            Direction::Left => Direction::Down,
                        };
                        let new_location = new_direction.along(&location);
                        let entry = new_carts.entry(new_location).or_insert(vec![]);
                        cart.borrow_mut().change_direction(new_direction);
                        entry.push(Rc::clone(cart));
                        let did_collide = entry.iter().filter(|c| c.borrow().is_alive).count() > 1;

                        if did_collide {
                            entry
                                .iter_mut()
                                .for_each(|c| c.borrow_mut().is_alive = false);
                        }

                        (did_collide, new_location)
                    }
                    Some(TrackType::Curve2) => {
                        // \
                        let new_direction = match cart.borrow().current_direction {
                            // \
                            // |
                            Direction::Up => Direction::Left,

                            // --\
                            Direction::Right => Direction::Down,

                            // |
                            // \
                            Direction::Down => Direction::Right,

                            // \--
                            Direction::Left => Direction::Up,
                        };
                        let new_location = new_direction.along(&location);
                        let entry = new_carts.entry(new_location).or_insert(vec![]);
                        cart.borrow_mut().change_direction(new_direction);
                        entry.push(Rc::clone(cart));
                        let did_collide = entry.iter().filter(|c| c.borrow().is_alive).count() > 1;

                        if did_collide {
                            entry
                                .iter_mut()
                                .for_each(|c| c.borrow_mut().is_alive = false);
                        }

                        (did_collide, new_location)
                    }

                    None => {
                        assert!(false, "Off the rails");
                        (false, (0, 0))
                    }
                };

                {
                    let entry = new_carts.entry(new_location).or_insert(vec![]);
                    if entry.iter().filter(|c| c.borrow().is_alive).count() > 1 {
                        for cart in entry {
                            cart.borrow_mut().is_alive = false;
                        }
                    }
                }

                {
                    let entry = new_carts.entry(location).or_insert(vec![]);
                    entry.clear();
                }

                if did_collide && halt_on_collision {
                    break 'outer;
                }
            }
        }

        self.carts = new_carts;
    }
}

impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .enumerate()
                .map(|(y, line)| line
                    .iter()
                    .enumerate()
                    .map(|(x, t)| {
                        let empty_vec = vec![];
                        let carts = self
                            .carts
                            .get(&(x, y))
                            .map(|carts| carts)
                            .unwrap_or(&empty_vec);

                        if carts.len() == 1 {
                            carts[0].borrow().current_direction.to_char()
                        } else if carts.len() > 1 {
                            'X'
                        } else {
                            t.as_ref().map(|x| x.to_char()).unwrap_or(' ')
                        }
                    }).collect::<String>()).collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl<'a> From<&'a str> for Track {
    fn from(input: &'a str) -> Self {
        let grid: Vec<Vec<(Option<TrackType>, Vec<Cart>)>> = input
            .lines()
            .map(|line| line.trim_end())
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.chars()
                    .map(|c| {
                        (
                            TrackType::parse(c),
                            Direction::parse(c)
                                .map(|dir| vec![Cart::new(dir)])
                                .unwrap_or(vec![]),
                        )
                    }).collect()
            }).collect();

        let carts = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, (_, carts))| {
                        (
                            (x, y),
                            carts
                                .clone()
                                .into_iter()
                                .map(|cart| Rc::new(RefCell::new(cart)))
                                .collect(),
                        )
                    }).collect::<Vec<(Location, Vec<Rc<RefCell<Cart>>>)>>()
            }).collect();

        Self {
            grid: grid
                .into_iter()
                .map(|row| row.into_iter().map(|(t, _)| t).collect())
                .collect(),
            carts,
        }
    }
}

pub fn star_one(input: &str) -> Location {
    let mut track = Track::from(input);

    while !track.has_crash() {
        track.tick(true);
    }

    track.crash_location().unwrap()
}

pub fn star_two(input: &str) -> Location {
    let mut track = Track::from(input);
    let mut ticks: Vec<String> = vec![];
    println!("Num alive at start: {}", track.num_alive_carts());

    loop {
        track.tick(false);
        let num_alive = track.num_alive_carts();

        if num_alive == 1 {
            break;
        }
        ticks.push(format!("{:?}", track));
        assert!(
            num_alive % 2 == 1,
            "There should alwasy be an odd number of live carts, but it was {}. Last ticks: \n{}",
            num_alive,
            ticks
                .iter()
                .skip(ticks.len() - 3)
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    track.alive_carts_locations()[0]
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    static EXAMPLE_ONE: &str = "
/->-\\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/
";
    static EXAMPLE_TWO: &str = "
/>-<\\
|   |
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/
";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE_ONE), (7, 3));
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(EXAMPLE_TWO), (6, 4));
    }
}
