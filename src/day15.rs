use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter;
use std::rc::Rc;

// x, y pair
type Location = (usize, usize);
type UnitPointer = Rc<RefCell<Unit>>;

fn reading_order(lhs: &Location, rhs: &Location) -> Ordering {
    let order = lhs.1.cmp(&rhs.1);
    if order != Ordering::Equal {
        order
    } else {
        lhs.0.cmp(&rhs.0)
    }
}

#[derive(Eq, PartialEq, Clone)]
enum UnitType {
    Elf,
    Goblin,
}

impl fmt::Debug for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::Elf => write!(f, "E"),
            UnitType::Goblin => write!(f, "G"),
        }
    }
}

#[derive(Debug, Clone)]
struct Unit {
    unit_type: UnitType,
    health: usize,
    strength: usize,
    is_dead: bool,
}

impl Unit {
    fn new(unit_type: UnitType) -> Self {
        Self {
            unit_type,
            health: 200,
            strength: 3,
            is_dead: false,
        }
    }

    fn to_char(&self) -> char {
        match self.unit_type {
            UnitType::Goblin => 'G',
            UnitType::Elf => 'E',
        }
    }

    fn take_damage(&mut self, damage: usize) -> bool {
        match self.health.overflowing_sub(damage) {
            (new_health, false) => {
                self.health = new_health;

                if new_health == 0 {
                    self.is_dead = true;
                    true
                } else {
                    false
                }
            }
            (_, true) => {
                self.is_dead = true;
                self.health = 0;
                true
            }
        }
    }

    fn is_alive(&self) -> bool {
        !self.is_dead
    }

    fn is_dead(&self) -> bool {
        self.is_dead
    }
}

enum Position {
    Wall,
    Open,
    Occupied(UnitPointer),
}

impl Position {
    fn parse(input: char) -> Option<Self> {
        match input {
            '#' => Some(Position::Wall),
            '.' => Some(Position::Open),
            'G' => Some(Position::Occupied(Rc::new(RefCell::new(Unit::new(
                UnitType::Goblin,
            ))))),
            'E' => Some(Position::Occupied(Rc::new(RefCell::new(Unit::new(
                UnitType::Elf,
            ))))),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Position::Wall => '#',
            Position::Open => '.',
            Position::Occupied(occupant) => occupant.borrow().to_char(),
        }
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        match self {
            Position::Wall => Position::Wall,
            Position::Open => Position::Open,
            Position::Occupied(occupant) => {
                Position::Occupied(Rc::new(RefCell::new(occupant.borrow().clone())))
            }
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Wall => write!(f, "#"),
            Position::Open => write!(f, "."),
            Position::Occupied(occupant) => write!(f, "{:?}", occupant.borrow()),
        }
    }
}

struct GameState {
    grid: Vec<Vec<Position>>,
    combatants: HashMap<Location, UnitPointer>,
}

impl<'a> From<&'a str> for GameState {
    fn from(input: &'a str) -> Self {
        let mut combatants = HashMap::new();

        Self {
            grid: input
                .lines()
                .map(|line| line.trim())
                .filter(|line| line.len() > 0)
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            let pos =
                                Position::parse(c).expect(&format!("Unexpected position {}", c));

                            match &pos {
                                Position::Occupied(occupant) => {
                                    combatants.insert((x, y), Rc::clone(&occupant));
                                }
                                _ => {}
                            };

                            pos
                        }).collect()
                }).collect(),
            combatants: combatants,
        }
    }
}

impl GameState {
    fn in_range<'a>(
        &'a self,
        location: &'a Location,
        only_open: bool,
    ) -> impl Iterator<Item = Location> + 'a {
        [(0, 1), (1, 0), (0, -1i64), (-1i64, 0)]
            .iter()
            .flat_map(move |direction| {
                if (location.0 == 0 && direction.0 == -1) || (location.1 == 0 && direction.1 == -1)
                {
                    None
                } else if (location.0 == self.grid[0].len() - 1 && direction.0 == 1)
                    || (location.1 == self.grid.len() - 1 && direction.1 == 1)
                {
                    None
                } else {
                    let x: usize = (location.0 as i64 + direction.0) as usize;
                    let y: usize = (location.1 as i64 + direction.1) as usize;

                    match self.grid[y][x] {
                        Position::Open => Some((x, y)),
                        Position::Occupied(_) => if !only_open {
                            Some((x, y))
                        } else {
                            None
                        },
                        _ => None,
                    }
                }
            })
    }

    fn prioritized_enemy(
        &self,
        unit: &Unit,
        unit_location: &Location,
    ) -> Option<(Location, UnitPointer)> {
        let mut enemies_in_range: Vec<(Location, UnitPointer)> = self
            .in_range(unit_location, false)
            .flat_map(|(x, y)| match self.combatants.get(&(x, y)) {
                Some(occupant) => {
                    if occupant.borrow().unit_type != unit.unit_type {
                        Some(((x, y), Rc::clone(&occupant)))
                    } else {
                        None
                    }
                }
                _ => None,
            }).collect();

        if enemies_in_range.is_empty() {
            None
        } else if enemies_in_range.len() == 1 {
            enemies_in_range.into_iter().nth(0)
        } else {
            enemies_in_range.sort_by(|(lhs_location, lhs), (rhs_location, rhs)| {
                let ordering = lhs.borrow().health.cmp(&rhs.borrow().health);

                if ordering != Ordering::Equal {
                    ordering
                } else {
                    reading_order(lhs_location, rhs_location)
                }
            });

            enemies_in_range.into_iter().nth(0)
        }
    }

    fn enemies_alive(&self, unit: &Unit) -> bool {
        match unit.unit_type {
            UnitType::Goblin => self.num_combatants_alive(UnitType::Elf) != 0,
            UnitType::Elf => self.num_combatants_alive(UnitType::Goblin) != 0,
        }
    }

    fn num_combatants_alive(&self, combatant_type: UnitType) -> usize {
        self.combatants.values().fold(0, |acc, unit| {
            if unit.borrow().unit_type == combatant_type && unit.borrow().is_alive() {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn possible_targets(&self, unit: &Unit) -> Vec<(Location, UnitPointer)> {
        self.combatants
            .iter()
            .filter(|(_, other_unit)| unit.unit_type != other_unit.borrow().unit_type)
            .map(|(location, other)| (location.clone(), Rc::clone(other)))
            .collect()
    }

    fn cheat(&self, new_elf_strength: usize) -> Self {
        let mut combatants = HashMap::new();
        let grid = self
            .grid
            .clone()
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.clone()
                    .into_iter()
                    .enumerate()
                    .map(|(x, pos)| {
                        let new_pos = pos.clone();

                        match pos {
                            Position::Occupied(occupant) => {
                                if occupant.borrow().unit_type == UnitType::Elf {
                                    occupant.borrow_mut().strength = new_elf_strength;
                                }

                                combatants.insert((x, y), Rc::clone(&occupant));
                            }
                            _ => {}
                        };

                        new_pos
                    }).collect()
            }).collect();

        Self { grid, combatants }
    }

    fn calculate_distance_grid(&self, from: &Location) -> Option<Vec<Vec<Option<usize>>>> {
        let mut possible_moves = self.in_range(from, true).collect::<Vec<_>>();
        possible_moves.sort_by(reading_order);

        let (x, y) = from.clone();
        let mut distance_grid: Vec<Vec<Option<usize>>> =
            vec![vec![None; self.grid[0].len()]; self.grid.len()];
        let mut visited: HashSet<Location> =
            HashSet::with_capacity(self.grid.len() * self.grid[0].len());

        distance_grid[y][x] = Some(0);
        visited.insert(from.clone());
        let mut to_visit: VecDeque<(Location, usize)> = VecDeque::new();
        let mut to_visit_set: HashSet<Location> = HashSet::new();
        for l in possible_moves.iter() {
            if !visited.contains(l) && !to_visit_set.contains(l) {
                to_visit.push_front((l.clone(), 1));
                to_visit_set.insert(l.clone());
            }
        }

        while !to_visit.is_empty() {
            let (current, distance) = to_visit.pop_back().unwrap();
            visited.insert(current);

            match self.grid[current.1][current.0] {
                Position::Open => {
                    distance_grid[current.1][current.0] = Some(distance);
                    for l in self.in_range(&current, true) {
                        if !visited.contains(&l) && !to_visit_set.contains(&l) {
                            to_visit.push_front((l, distance + 1));
                            to_visit_set.insert(l.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        Some(distance_grid)
    }

    fn first_move_on_shortest_path(
        &self,
        unit_poistion: &Location,
        to: &Location,
    ) -> Option<Location> {
        match self.calculate_distance_grid(to) {
            None => None,
            Some(distance_grid) => {
                let mut possible_moves = self.in_range(&unit_poistion, true).collect::<Vec<_>>();
                possible_moves.sort_by(|lhs, rhs| {
                    let order = distance_grid[lhs.1][lhs.0].cmp(&distance_grid[rhs.1][rhs.0]);

                    if order != Ordering::Equal {
                        order
                    } else {
                        reading_order(lhs, rhs)
                    }
                });

                possible_moves
                    .into_iter()
                    .filter(|x| distance_grid[x.1][x.0].is_some())
                    .nth(0)
                    .map(|x| x)
            }
        }
    }

    fn turn(&mut self) -> (bool, Option<UnitType>) {
        let mut unit_locations: Vec<(Location, UnitPointer)> = self
            .combatants
            .iter()
            .map(|(l, combatant)| (l.clone(), Rc::clone(combatant)))
            .collect();
        unit_locations.sort_by(|(a, _), (b, _)| reading_order(a, b));

        for (unit_location, unit) in unit_locations.into_iter() {
            if !self.enemies_alive(&unit.borrow()) {
                return (false, Some(unit.borrow().unit_type.clone()));
            }

            if unit.borrow().is_dead() {
                continue;
            }

            let enemy = self.prioritized_enemy(&unit.borrow(), &unit_location);

            if enemy.is_some() {
                let (enemy_location, e) = enemy.unwrap();
                let died = e.borrow_mut().take_damage(unit.borrow().strength);

                if died {
                    self.combatants.remove(&enemy_location);
                    self.grid[enemy_location.1][enemy_location.0] = Position::Open;
                }
            } else {
                let possible_targets = self.possible_targets(&unit.borrow());

                if possible_targets.is_empty() {
                    continue;
                }

                let potential_distance_grid = self.calculate_distance_grid(&unit_location);
                if potential_distance_grid.is_none() {
                    continue;
                }

                let distance_grid = potential_distance_grid.unwrap();

                let mut possible_targets_with_distance = possible_targets
                    .iter()
                    .flat_map(|(enemy_location, _)| self.in_range(enemy_location, true))
                    .flat_map(|target_location| {
                        match distance_grid[target_location.1][target_location.0] {
                            None => None,
                            Some(distance) => Some((target_location, distance)),
                        }
                    }).collect::<Vec<(Location, usize)>>();

                if possible_targets_with_distance.is_empty() {
                    continue;
                }

                possible_targets_with_distance.sort_by(|(_, lhs_distance), (_, rhs_distance)| {
                    lhs_distance.cmp(&rhs_distance)
                });
                let shortest_distance = possible_targets_with_distance[0].1;

                let mut possible_first_moves = possible_targets_with_distance
                    .iter()
                    .filter(|(_, distance)| *distance == shortest_distance)
                    .flat_map(|(location, _)| {
                        self.first_move_on_shortest_path(&unit_location, &location)
                            .map(|move_to| (location, move_to))
                    }).collect::<Vec<_>>();

                possible_first_moves.sort_by(|(lhs, _), (rhs, _)| reading_order(lhs, rhs));

                possible_first_moves
                    .into_iter()
                    .nth(0)
                    .iter()
                    .for_each(|(_, new_location)| {
                        // Delete old location
                        self.combatants.remove(&unit_location);
                        self.grid[unit_location.1][unit_location.0] = Position::Open;

                        // Add new location
                        self.grid[new_location.1][new_location.0] =
                            Position::Occupied(Rc::clone(&unit));

                        self.combatants
                            .insert(new_location.clone(), Rc::clone(&unit));

                        let new_enemy = self.prioritized_enemy(&unit.borrow(), &new_location);

                        if new_enemy.is_some() {
                            let (new_enemy_location, ne) = new_enemy.unwrap();
                            let died = ne.borrow_mut().take_damage(unit.borrow().strength);

                            if died {
                                self.combatants.remove(&new_enemy_location);
                                self.grid[new_enemy_location.1][new_enemy_location.0] =
                                    Position::Open;
                            }
                        }
                    });
            }
        }

        let (goblins_left, elves_left) = (
            self.num_combatants_alive(UnitType::Goblin),
            self.num_combatants_alive(UnitType::Elf),
        );

        if goblins_left == 0 || elves_left == 0 {
            if goblins_left == 0 {
                (true, Some(UnitType::Elf))
            } else {
                (true, Some(UnitType::Goblin))
            }
        } else {
            (true, None)
        }
    }

    fn remaining_health_for_faction(&self, faction: UnitType) -> usize {
        self.combatants.values().fold(0, |acc, unit| {
            let borrowed_unit = unit.borrow();
            if borrowed_unit.unit_type == faction && borrowed_unit.is_alive() {
                acc + borrowed_unit.health
            } else {
                acc
            }
        })
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(|pos| pos.to_char()).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub fn star_one(input: &str) -> usize {
    let mut state = GameState::from(input);
    let (completed_turns, winning_faction) = iter::repeat(0)
        .enumerate()
        .map(|(id, _)| {
            let (_, turn_result) = state.turn();

            (id, turn_result)
        }).skip_while(|(_, turn_result)| turn_result.is_none())
        .nth(0)
        .map(|(turns, end_result)| (turns, end_result))
        .unwrap();

    completed_turns * state.remaining_health_for_faction(winning_faction.unwrap())
}

pub fn star_two(input: &str) -> usize {
    let initial_state = GameState::from(input);
    let number_of_elves_in_combat = initial_state.num_combatants_alive(UnitType::Elf);

    let (completed_turns, adjusted_strength, winning_faction, final_state) = iter::repeat(0)
        .enumerate()
        .map(|(strength_increase, _)| {
            let adjusted_strength = 4 + strength_increase;
            let mut state = initial_state.cheat(adjusted_strength);

            let (completed_turns, winning_faction) = iter::repeat(0)
                .enumerate()
                .map(|(id, _)| {
                    let (full_turn, turn_result) = state.turn();

                    if state.num_combatants_alive(UnitType::Elf) < number_of_elves_in_combat {
                        (id, Some(UnitType::Goblin))
                    } else {
                        let turn_count = if full_turn { id + 1 } else { id };
                        (turn_count, turn_result)
                    }
                }).skip_while(|(_, turn_result)| turn_result.is_none())
                .nth(0)
                .unwrap();

            (
                completed_turns,
                adjusted_strength,
                winning_faction.unwrap(),
                Some(state),
            )
        }).skip_while(|(_, _, turn_result, _)| turn_result == &UnitType::Goblin)
        .nth(0)
        .unwrap();

    completed_turns * final_state
        .unwrap()
        .remaining_health_for_faction(winning_faction)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_ONE: &str = "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    static EXAMPLE_TWO: &str = "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    static EXAMPLE_THREE: &str = "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    static EXAMPLE_FOUR: &str = "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    static EXAMPLE_FIVE: &str = "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    static EXAMPLE_SIX: &str = "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
    static EXAMPLE_SEVEN: &str = "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
    static EXAMPLE_EIGHT: &str = "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    static EXAMPLE_NINE: &str = "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    static EXAMPLE_TEN: &str = "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE_ONE), 36334);
        assert_eq!(star_one(EXAMPLE_TWO), 39514);
        assert_eq!(star_one(EXAMPLE_THREE), 27755);
        assert_eq!(star_one(EXAMPLE_FOUR), 28944);
        assert_eq!(star_one(EXAMPLE_FIVE), 18740);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(EXAMPLE_SIX), 4988);
        assert_eq!(star_two(EXAMPLE_SEVEN), 31284);
        assert_eq!(star_two(EXAMPLE_EIGHT), 3478);
        assert_eq!(star_two(EXAMPLE_NINE), 6474);
        assert_eq!(star_two(EXAMPLE_TEN), 1140);
    }

    #[test]
    fn first_move_on_shortest_path() {
        let input = "
#######
#.E...#
#.....#
#...G.#
#######";
        let state = GameState::from(input);

        let mut move_to_make = state.first_move_on_shortest_path(&(2, 1), &(4, 2));
        assert_eq!(move_to_make, Some((3, 1)));

        move_to_make = state.first_move_on_shortest_path(&(4, 2), &(2, 1));
        assert_eq!(move_to_make, Some((4, 1)));
    }

    #[test]
    fn first_move_on_shortest_path_edge_case() {
        let input = "#######
#G.E#E#
#E#..E#
#G.##.#
#.E.#.#
#....E#
#######";
        let state = GameState::from(input);
        let move_to_make = state.first_move_on_shortest_path(&(2, 4), &(2, 3));

        assert_eq!(move_to_make, Some((2, 3)));
    }

    #[test]
    fn test_reading_order() {
        assert_eq!(reading_order(&(2, 3), &(1, 4)), Ordering::Less);
    }
}
