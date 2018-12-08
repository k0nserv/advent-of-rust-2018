use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, x: i64, y: i64) -> i64 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

impl<'a> From<&'a str> for Point {
    fn from(input: &'a str) -> Self {
        let parts: Vec<i64> = input
            .split(',')
            .map(|part| {
                part.trim()
                    .parse::<i64>()
                    .expect("Expected parsable numbers")
            }).collect();
        assert!(
            parts.len() == 2,
            "Each point should have exactly two coordinates"
        );

        Self {
            x: parts[0],
            y: parts[1],
        }
    }
}

pub fn find_extremes(points: &[Point]) -> (Point, Point) {
    let max = Point::new(
        points.iter().max_by_key(|p| p.x).unwrap().x,
        points.iter().max_by_key(|p| p.y).unwrap().y,
    );
    let min = Point::new(
        points.iter().min_by_key(|p| p.x).unwrap().x,
        points.iter().min_by_key(|p| p.y).unwrap().y,
    );

    (max, min)
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Point> + 'a {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(Point::from)
}

pub struct Grid<T> {
    max: Point,
    min: Point,
    data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    fn new_with_corners(max: &Point, min: &Point, padding: i64) -> Self {
        let padded_max = Point::new(max.x + padding, max.y + padding);
        let padded_min = Point::new(min.x - padding, min.y - padding);

        let height = padded_max.y - padded_min.y + 1;
        let width = padded_max.x - padded_min.x + 1;
        let data = vec![vec![T::default(); height as usize]; width as usize];

        Self {
            max: padded_max,
            min: padded_min,
            data,
        }
    }
}

impl<T> Grid<T> {
    fn ranges(&self) -> (Range<i64>, Range<i64>) {
        (
            (self.min.x..(self.max.x + 1)),
            (self.min.y..(self.max.y + 1)),
        )
    }
}

impl<T> Index<(i64, i64)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &T {
        &self.data[(index.0 - self.min.x) as usize][(index.1 - self.min.y) as usize]
    }
}

impl<T> IndexMut<(i64, i64)> for Grid<T> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut T {
        &mut self.data[(index.0 - self.min.x) as usize][(index.1 - self.min.y) as usize]
    }
}

#[derive(Clone)]
enum Location<'a> {
    Unspecified,
    Nearest(&'a Point),
    EquallyFar,
}

impl<'a> Default for Location<'a> {
    fn default() -> Self {
        Location::Unspecified
    }
}

fn fill_grid<'a, 'b>(grid: &'a mut Grid<Location<'b>>, points: &'b [Point]) {
    let (x_range, y_range) = grid.ranges();

    for x in x_range.clone() {
        for y in y_range.clone() {
            let mut distance = i64::max_value();
            let mut location = Location::Unspecified;

            for point in points {
                let distance_to_point = point.manhattan_distance(x, y);

                if distance_to_point < distance {
                    location = Location::Nearest(point);
                    distance = distance_to_point;
                } else if distance_to_point == distance {
                    match location {
                        Location::Unspecified => {
                            location = Location::Nearest(point);
                            distance = distance_to_point;
                        }
                        Location::Nearest(_) => {
                            location = Location::EquallyFar;
                        }
                        _ => (),
                    }
                }
            }

            grid[(x, y)] = location;
        }
    }
}

pub fn star_one(input: &str) -> i64 {
    let points = parse(input).collect::<Vec<_>>();
    let (max, min) = find_extremes(&points);
    let mut grid = Grid::<Location>::new_with_corners(&max, &min, 1);
    let mut potential_points: Vec<Option<Point>> =
        points.clone().into_iter().map(Option::Some).collect();
    let (x_range, y_range) = grid.ranges();
    fill_grid(&mut grid, &points);

    // Remove outermost points as they escape to infinity by definition
    // Top and bottom edges
    for x in x_range.clone() {
        for y in [y_range.start, y_range.end - 1].iter() {
            match grid[(x.clone(), y.clone())] {
                Location::Unspecified => (),
                Location::EquallyFar => (),
                Location::Nearest(point) => {
                    let idx = potential_points
                        .iter()
                        .position(|p| p.as_ref().map(|x| x == point).unwrap_or(false));

                    idx.into_iter().for_each(|idx| potential_points[idx] = None);
                }
            }
        }
    }

    // Remove outermost points as they escape to infinity by definition
    // Left and right edges
    for y in y_range.clone() {
        for x in [x_range.start, x_range.end - 1].iter() {
            match grid[(x.clone(), y.clone())] {
                Location::Unspecified => (),
                Location::EquallyFar => (),
                Location::Nearest(point) => {
                    let idx = potential_points
                        .iter()
                        .position(|p| p.as_ref().map(|x| x == point).unwrap_or(false));

                    idx.into_iter().for_each(|idx| potential_points[idx] = None);
                }
            }
        }
    }

    let unescaped_points: Vec<Point> = potential_points.into_iter().flat_map(|x| x).collect();
    let mut area_sizes = vec![];

    for point in unescaped_points {
        let mut area = 0;

        for x in x_range.clone() {
            for y in y_range.clone() {
                match grid[(x.clone(), y.clone())] {
                    Location::Unspecified => {
                        assert!(false, "Should not still happen");
                        ()
                    }
                    Location::EquallyFar => (),
                    Location::Nearest(grid_point) => {
                        if &point == grid_point {
                            area += 1;
                        }
                    }
                }
            }
        }

        area_sizes.push(area);
    }

    area_sizes.into_iter().max().unwrap()
}

pub fn star_two(input: &str, target_distance: i64) -> i64 {
    let points = parse(input).collect::<Vec<_>>();
    let (max, min) = find_extremes(&points);
    let mut grid = Grid::<Location>::new_with_corners(&max, &min, 1);
    let (x_range, y_range) = grid.ranges();
    fill_grid(&mut grid, &points);

    let mut count = 0;
    for x in x_range.clone() {
        for y in y_range.clone() {
            let result = points.iter().fold(Some(0), |acc, point| match acc {
                None => acc,
                Some(sum) => {
                    let distance = point.manhattan_distance(x, y);
                    if sum + distance < target_distance {
                        Some(sum + distance)
                    } else {
                        None
                    }
                }
            });

            result.iter().for_each(|_| count += 1);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &'static str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE), 17);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(EXAMPLE, 32), 16)
    }

    #[test]
    fn grid_construction() {
        let points = parse(EXAMPLE).collect::<Vec<_>>();
        let (max, min) = find_extremes(&points);
        let grid = Grid::<i64>::new_with_corners(&max, &min, 1);

        let (x_range, y_range) = grid.ranges();

        assert_eq!(x_range, (0..10));
        assert_eq!(y_range, (0..11));
    }
}
